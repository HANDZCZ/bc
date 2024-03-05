
## Porovnání frameworků pro vývoj backendu

Hlavní porovnávané frameworky jsou express, axum, actix, rocket, fastapi, django, flask
a php, které bylo přidáno jen kvůli porovnání, ale nebylo nikdy uvažováno nad jejím použitím.
Hlavním důvodem je, že je to zastaralá technologie, která už dávno neměla existovat,
ale i přesto je dále vyučována a používána.

Bylo provedeno porovnání funkcí těchto frameworků ([@tbl:framework_features_comparison]).
Porovnány byly také podle toho, kolik dotazů za sekundu zvládnou ([@tbl:framework_comparison_requests_per_second_table; @fig:framework_comparison_requests_per_second_graph]).
A nakonec byly porovnány podle latencí.
Průměrné latence frameworků jsou uvedeny v tabulce [-@tbl:framework_comparison_average_latency_table] a znázorněny grafem v obrázku [-@fig:framework_comparison_average_latency_graph].
Latence P99 jsou uvedeny v tabulce [-@tbl:framework_comparison_p99_latency_table] a znázorněny grafem v obrázku [-@fig:framework_comparison_p99_latency_graph].
Stejně tak jsou latence P90 uvedeny v tabulce [-@tbl:framework_comparison_p90_latency_table] a znázorněny grafem v obrázku [-@fig:framework_comparison_p90_latency_graph]
a jako poslední byly porovnány latence P75, které jsou uvedeny v tabulce [-@tbl:framework_comparison_p75_latency_table] a znázorněny grafem v obrázku [-@fig:framework_comparison_p75_latency_graph].

Tyto tabulky a grafy byly vytvořeny pomocí autorem napsaných skriptů,
které lze najít [zde](https://github.com/HANDZCZ/bc/tree/main/thesis/utils).
Tyto skripty je poté nutné vložit do konzole prohlížeče na stránce [Web Frameworks Benchmark](https://web-frameworks-benchmark.netlify.app/result).
Pro aplikaci stejných filtrů je nutno použít stejný odkaz jako je uveden ve zdroji [-@web_frameworks_benchmark_tables].
