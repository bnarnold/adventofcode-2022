set dotenv-load

year := '2022'
env_day := env_var_or_default('DAY',`date '+%d'`)

run level='1' day=env_day:
  cargo run --example day{{day}} -- --level {{level}}

submit level='1' day=env_day:
  cargo run --example day{{day}} -- --level {{level}} --submit

download day=env_day:
  curl https://adventofcode.com/{{year}}/day/{{day}}/input -H "Cookie: session=$SESSION" -o "./input/day{{day}}.txt"

paste day=env_day:
  xclip -o -selection c > src/days/test_input/day{{day}}.txt

generate day=env_day:
  printf "pub mod day{{day}};\n" >> src/days/mod.rs
  cat templates/library | sed -e s/##DAY##/{{day}}/g > src/days/day{{day}}.rs
  mkdir -p examples/day{{day}}
  cat templates/example | sed -e s/##DAY##/{{day}}/g > examples/day{{day}}/main.rs
  

