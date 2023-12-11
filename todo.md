change the remove patient fn to make the patient none insteaf of removing from the vec

change the next patient fn to skip None values

change the win condition to accomidate


add music


to release:

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./files/out/ --target web ./target/wasm32-unknown-unknown/release/bevy-jam.wasm


bash script.sh