# Schwab API Issues

## 1. `currentDayProfitLoss` on positions returns incorrect values

The `currentDayProfitLoss` field on the Position object returns values that don't correspond to the actual day's price movement on the position. For example, a large-cap equity position once reported a day P&L of roughly -$15,000 even though the underlying had only moved about $2 intraday (which should be on the order of a few hundred dollars), with no other trades on that symbol that day.

**Workaround:** Compute day P&L manually from `(lastPrice - closePrice) * quantity` using streaming/quote data.

## 2. `SPECIFIC_LOT` tax lot method exists but no field to specify which lot

The `taxLotMethod` enum on orders includes a `SPECIFIC_LOT` value, but the order schema (`OrderLegCollection`) has no corresponding field to pass an actual lot ID. So you can tell the API you want to sell a specific lot, but there's no way to say which one.

## 3. `MarginBalance` schema missing fields that the API actually returns

The OpenAPI spec for `MarginBalance` (used in `currentBalances` and `projectedBalances` on margin accounts) is missing several fields that the API actually returns in the JSON response. Notable missing fields:

- `cashBalance` — the actual cash balance in the account
- `liquidationValue` — total liquidation value
- `accruedInterest`, `bondValue`, `cashReceipts`, `longMarketValue`, `shortMarketValue`, `shortOptionMarketValue`, `longOptionMarketValue`, `moneyMarketFund`, `mutualFundValue`, `pendingDeposits`, `savings`

These fields are present in the `MarginInitialBalance` schema but absent from `MarginBalance`, even though the API returns them on `currentBalances`. Without `cashBalance`, there is no way to get the current cash balance from the typed response — only `availableFunds` is available, which represents margin-adjusted available funds, not actual cash.

## 4. Option chain `underlying.exchangeName` returns full names, not the spec's enum

The `get_chain` response embeds an `underlying` quote whose `exchangeName` is a full exchange name (e.g. `"NASDAQ"`), but the OpenAPI `Underlying.exchangeName` is an enum of short codes (`IND`/`ASE`/`NYS`/`NAS`/`NAP`/`PAC`/`OPR`/`BATS`). The value `"NASDAQ"` is not in the enum, so deserialization of the **entire** chain fails with "Invalid Response Payload". (Note the per-contract `exchangeName` *does* use short codes like `OPR`, so only the underlying field is affected.)

**Fix:** patched `Underlying.exchange_name` to `Option<String>` in `schwab-marketdata` to tolerate any exchange name.

## 5. Transactions API never reports which lots/method a sale used

There is no way to determine, *from a transaction alone*, which tax lot method
(FIFO, LIFO, specific lot, etc.) was applied to a closing trade, or which
acquisition lots were disposed of:

- The `Transaction` object (`GET /accounts/{id}/transactions`) has no
  `taxLotMethod` field. Its `transferItems` give per-trade `amount`, `cost`,
  `price`, and `positionEffect`, but no lot-level disposition — nothing ties a
  sale back to the specific acquisition lots it closed, and nothing states the
  matching method used. (Re-verified 2026-06-03 against live sell transactions:
  see `test_transactions_omit_lot_method`.)
- Schwab's website transaction-history CSV export doesn't include the lot method
  either, so there's no out-of-band source.

**However**, `taxLotMethod` *is* echoed back on the **order** object — both on a
working order and after fill (`GET /accounts/{id}/orders`), not only on the
request side. Verified 2026-06-03 (`test_order_echoes_tax_lot_method`): a sell
placed with `taxLotMethod = FIFO` reads back with `taxLotMethod: Fifo`.

Net effect: the method can be recovered by **joining a transaction to its order**
via `transaction.orderId → order.taxLotMethod`, as long as the order is still
within Schwab's orders query window (~60 days). For older fills, or to know the
*specific lots* disposed (still never reported anywhere), we must track lots and
apply our own matching method locally.

No way to trade EXTO?
Is there any documentation on how wash sales work
