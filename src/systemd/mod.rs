use serde::{Deserialize, Serialize};

mod zbusproxy;

pub type Weight = u16;

#[derive(Debug, Serialize, Deserialize)]
pub struct Policy {
    pub name: String,

    // CPU control parameters
    // A parent's resource is distributed by adding up the weights of all active children and giving each the fraction matching the ratio of its weight against the sum.
    pub cpu_weight: Option<Weight>,
    pub allowed_cpus: Option<Vec<u64>>,

    // Memory protection parameters
    pub memory_min: Option<u64>,
    pub memory_low: Option<u64>,

    // Memory limit parameters
    pub memory_high: Option<u64>,
    pub memory_max: Option<u64>,
    pub memory_swap_max: Option<u64>,
    pub memory_zswap_max: Option<u64>,

    // IO control parameters
    pub io_weight: Option<u64>,
    pub io_device_weight: Option<(String, u64)>,

    // IO limit parameters
    pub io_max: Option<u64>,

    // Task limit parameters
    pub task_max: Option<u64>, // TODO: Memory Pressure Control
}


trait Slice {}

trait UserOrService {}

//
//                      -.slice
//                     /       \
//              /-----/         \--------------\
//             /                                \
//      system.slice                       user.slice
//        /       \                          /      \
//       /         \                        /        \
//      /           \              user@42.service  user@1000.service
//     /             \             Delegate=        Delegate=yes
//a.service       b.slice                             /        \
//CPUWeight=20   DisableControllers=cpu              /          \
//                 /  \                      app.slice      session.slice
//                /    \                     CPUWeight=100  CPUWeight=100
//               /      \
//       b1.service   b2.service
//                    CPUWeight=1000

// The name consists of a dash-separated series of names, which describes the
// path to the slice from the root slice.
// The root slice is named -.slice.
// Example: foo-bar.slice is a slice that is located within foo.slice,
// which in turn is located in the root slice -.slice. 

// What do I wanna do

// 1. essential.slice - this is static and can just be a file and specify it in unitfile
// For example, every user gets their own slice user-nnn.slice.
// 2. Make a user assignee to resources by moving its slice under user-asginee.slice
// 3. Unasign previous asignee
// 4. Make RAM reservation.
// 5. Get status: how much RAM reserved


fn new_slice(parent: &str) -> String{
    unimplemented!()
}

fn assign_policy(thing: impl UserOrService, policy: &Policy) -> String{
    unimplemented!()
}

fn get_policy(thing: impl UserOrService) -> Policy{
    unimplemented!()
}

