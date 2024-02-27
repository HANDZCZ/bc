
## Spuštění pomocí kontejnerů

Pro spuštění aplikace pomocí kontejnerů je potřeba mít nainstalovaný nějaký software,
který umožní spouštění a manipulaci kontejnerů, jako je například Docker.

Nejprve je potřeba naklonovat kódový repozitář.
K naklonování kódového repozitáře je potřeba mít nainstalovaní git.
Poté jen stačí naklonovat git repozitář pomocí příkazu níže.

```{.bash}
git clone https://github.com/HANDZCZ/bc.git
```

: Příkaz pro naklonování kódového repozitáře {#lst:git_clone_command}

Po naklonování kódového repozitáře je potřeba zkopírovat příklad docker-compose souboru a přejmenovat ho na docker-compose.yaml.
Poté je potřeba tento soubor upravit podle komentářů uvedených v tomto souboru.

Na konec už jen stačí spustit aplikaci pomocí příkazu níže.

```{.bash}
docker compose up -d
```

: Příkaz pro spuštění kontejnerizované aplikace {#lst:docker_compose_up_command}

Pokud je na systému nainstalovaný software make stačí spustit příkaz níže.

```{.bash}
make run
```

: Příkaz pro spuštění kontejnerizované aplikace pomocí make {#lst:make_run_command}

Dále lze také využít dalších pomocných příkazů jako je zastavení aplikace (``make stop``),
zastavení aplikace a odstranění dat vlastněných touto aplikací (``make nuke``)
a zastavení aplikace, odstranění dat vlastněných touto aplikací a spuštění aplikace (``make run-clean``).

