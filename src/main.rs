use serde::{Serialize, Deserialize};
use kube::core::{Resource, CustomResourceExt};
use kube_derive::CustomResource;
use schemars::JsonSchema;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(group = "clux.dev", version = "v1", kind = "Foo", namespaced)]
struct FooSpec {
    info: String,
}

fn main() {
    println!("kind = {}", Foo::kind(&())); // impl kube::Resource
    let f = Foo::new("foo-1", FooSpec {
        info: "informative info".into(),
    });
    println!("foo: {:?}", f); // debug print on root type
    println!("crd: {}", serde_yaml::to_string(&Foo::crd()).unwrap()); // crd yaml
}
