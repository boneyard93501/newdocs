## Remote Data Storage

### IPFS


[IPFS](https://ipfs.tech/) is a critical component of the Web3 ecosystem and can be considered distributed "hot" storage as opposed to Arweave's or Filecoin's "cold" storage and a great companion to decentralized, serverless compute. Hence, it shouldn't come as a surprise Fluence Rust peers provide IPFS sidecars as IPFS is used throughout the Fluence stack including service uploads for deployment. Moreover, Fluence provides the a subset of the cli commands as Marine services and corresponding Aqua bindings, which are documented in the [Aqua book](https://fluence.dev/docs/aqua-book/libraries/aqua-ipfs).

**todo: link to Aqua examples of using IPFS, e.g., module upload**


We can also use IPFS to manage function parameters. In that case, we need to create custom logic to handle our  


Regardless of how the host peer manages its IPFS resources, in order to be able to interact with IPFS the cli binary needs to be exposed. And just like with the curl adapter earlier, we need to create an IPFS adapter that allows us to access the host system, if permissioned by the host.

Create a new project with, e.g., `fluence init x-ipfs` and add a new (facade) service with `fluence service new` and let's call this the *ipfs_facade* module followed by adding the effector module, let's call it *ipfs_adapter*,  we need for our adapter with `fluence module new`.

Replace the template code in `services/modules/ipfs_adapter/src/man.rs` with the FFI host import and add a wrapper function around the *ipfs* function, just like we did with the curl adapter:

```rust=
use marine_rs_sdk::{marine, MountedBinaryResult};


fn main() {}

#[marine]
pub fn ipfs_request(cmd: Vec<String>) -> MountedBinaryResult {
    ipfs(cmd)
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn ipfs(cmd: Vec<String>) -> MountedBinaryResult;
}
```

Once again, we use Rust's [FFi](https://doc.rust-lang.org/nomicon/ffi.html) to link the host system to the Wasi module and specify the function *ipfs* to call the target binary and return the result(s) as MountedBinaryResult. See the [marine book](https://fluence.dev/docs/marine-book/marine-runtime/mounted-binaries) for additional information on how a host binary is linked to our adapter (effector) module. We wrap the linked *ipfs* function, which exposes the linked binary to the module, with the publicly visible and callable *ipfs_request* function.

We also need to update update the corresponding *module.yaml config file to associate the actual binary path, `/usr/bin/ipfs`, with our import function alias *ipfs*:

```yaml
version: 0
type: rust
name: ipfs_adapter
mountedBinaries:
  ipfs: /usr/bin/ipfs
```

Note that `/usr/bin/ipfs` is the path for the Fluence reference peers. On your local machine, you may find a different path such as `/usr/local/bin/ipfs` on a mac. Keep this in mind when setting up the *Config.toml* to work with the local Marine REPL, for example. 

Now that we have our adapter, we can proceed to write our facade module, i.e., our business logic that utilizes IPFS storage in some shape or form. Of course, we need a use case!  For the purpose of this 




** which ipfs examples should we link? is indexer still maintained and working ?**


### Ceramic ComposeDB

Coming soon.
