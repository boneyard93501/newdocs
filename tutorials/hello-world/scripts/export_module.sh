mkdir -p services/module-exports/modules/hello_world
fluence build
cd services/module-exports


cp ../modules/hello_world/target/wasm32-wasi/release/hello_world.wasm modules/hello_world/
cp ../modules/hello_world/module.yaml modules/hello_world/

tar -czvf hello_fluence.tar.gz  modules

rm -r modules
cd ../..