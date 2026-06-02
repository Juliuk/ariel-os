# STM32L496G-DISCO

## References

- [Manufacturer link](https://web.archive.org/web/20260305143452/https://www.st.com/en/evaluation-tools/32l496gdiscovery.html)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).

### `stm32l496g-disco`

- **Tier:** 1
- **Chip:** [STM32L496AG](../chips/stm32l496ag.md)
- **Chip Ariel OS Name:** `stm32l496ag`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b stm32l496g-disco
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Channel|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="needs testing">🚦</span>|
|SPI Main Mode|<span title="needs testing">🚦</span>|
|UART|<span title="needs testing">🚦</span>|
|Ethernet|<span title="not available on this piece of hardware">–</span>|
|User USB|<span title="needs testing">🚦</span>|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="supported with some caveats">☑️</span>[^removing-items-not-supported]|

<p>Legend:</p>

<dl>
  <div>
    <dt>✅</dt><dd>supported</dd>
  </div>
  <div>
    <dt>☑️</dt><dd>supported with some caveats</dd>
  </div>
  <div>
    <dt>🚦</dt><dd>needs testing</dd>
  </div>
  <div>
    <dt>❌</dt><dd>available in hardware, but not currently supported by Ariel OS</dd>
  </div>
  <div>
    <dt>–</dt><dd>not available on this piece of hardware</dd>
  </div>
</dl>
<style>
dt, dd {
  display: inline;
}
</style>


  
[^removing-items-not-supported]: Removing items not supported.