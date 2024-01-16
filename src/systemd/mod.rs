use ambassador::{delegatable_trait, Delegate};
use serde::{Deserialize, Serialize};
use zbus;
use zbus::{
    dbus_proxy,
    zvariant::ObjectPath,
};

type Properties = Vec<(String, String)>;

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

#[delegatable_trait]
trait SystemdUnit {
    fn set_properties(&self, runtime: bool, properties: Properties) -> zbus::Result<()>;
    fn id(&self) -> zbus::Result<String>;
    fn transient(&self) -> zbus::Result<bool>;
}

#[delegatable_trait]
trait SystemdService {
    fn slice(&self) -> zbus::Result<String>;
}
#[delegatable_trait]
trait SystemdSlice {
    fn slice(&self) -> zbus::Result<String>;
    fn control_group(&self) -> zbus::Result<String>;
    fn cpu_accounting(&self) -> zbus::Result<bool>;
    fn cpu_shares(&self) -> zbus::Result<u64>;
    fn block_io_accounting(&self) -> zbus::Result<bool>;
    fn block_io_weight(&self) -> zbus::Result<u64>;
    fn block_io_device_weight(&self) -> zbus::Result<Vec<(String, u64)>>;
    fn block_io_read_bandwidth(&self) -> zbus::Result<Vec<(String, u64)>>;
    fn block_io_write_bandwidth(&self) -> zbus::Result<Vec<(String, u64)>>;
    fn memory_accounting(&self) -> zbus::Result<bool>;
    fn memory_limit(&self) -> zbus::Result<u64>;
    fn device_policy(&self) -> zbus::Result<String>;
    fn device_allow(&self) -> zbus::Result<Vec<(String, String)>>;
}


#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Unit",
    default_service = "org.freedesktop.systemd1",
)]
trait Systemd1Unit {
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn transient(&self) -> zbus::Result<bool>;

    fn set_properties(&self, runtime: bool, properties: Properties) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Slice",
    default_service = "org.freedesktop.systemd1",
)]
trait Systemd1Slice {

    #[dbus_proxy(property)]
    fn slice(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn control_group(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn cpu_accounting(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn cpu_shares(&self) -> zbus::Result<u64>;

    #[dbus_proxy(property)]
    fn block_io_accounting(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn block_io_weight(&self) -> zbus::Result<u64>;

    #[dbus_proxy(property)]
    fn block_io_device_weight(&self) -> zbus::Result<Vec<(String, u64)>>;

    #[dbus_proxy(property)]
    fn block_io_read_bandwidth(&self) -> zbus::Result<Vec<(String, u64)>>;

    #[dbus_proxy(property)]
    fn block_io_write_bandwidth(&self) -> zbus::Result<Vec<(String, u64)>>;

    #[dbus_proxy(property)]
    fn memory_accounting(&self) -> zbus::Result<bool>;

    #[dbus_proxy(property)]
    fn memory_limit(&self) -> zbus::Result<u64>;

    #[dbus_proxy(property)]
    fn device_policy(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn device_allow(&self) -> zbus::Result<Vec<(String, String)>>;
}

#[dbus_proxy(
    name = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1",
)]
trait Systemd1Manager {
    #[dbus_proxy(property)]
    fn version(&self) -> zbus::Result<String>;

    #[dbus_proxy(signal)]
    fn UnitNew(&self, name: &str, unit: ObjectPath<'_>) -> zbus::Result<()>;

    fn SetUnitProperties(
        &self,
        name: &str,
        runtime: bool,
        properties: Properties,
    ) -> zbus::Result<()>;

    fn get_default_target(&self) -> zbus::Result<String>;

    #[dbus_proxy(object = "Systemd1Unit")]
    fn get_unit(&self, name: &str);
}

#[derive(Delegate)]
#[delegate(SystemdUnit, target = "unit")]
#[delegate(SystemdSlice, target = "slice")]
struct SliceUnitProxy<'a> {
    unit: Systemd1UnitProxyBlocking<'a>,
    slice: Systemd1SliceProxyBlocking<'a>,
}

impl TryFrom<Systemd1UnitProxyBlocking<'_>> for SliceUnitProxy<'_> {
    type Error = zbus::Error;
    fn try_from(u: Systemd1UnitProxyBlocking<'_>) -> Result<Self, Self::Error> {
        Ok(Self { unit: u,
            slice: Systemd1SliceProxyBlocking::builder(&u.connection())
            .path(u.path().to_owned())?
            .build()?})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zbus::blocking::Connection;

    fn setup<'a>() -> zbus::Result<(Connection, Systemd1ManagerProxyBlocking<'a>)> {
        let conn = Connection::system()?;
        let proxy = Systemd1ManagerProxyBlocking::new(&conn)?;
        Ok((conn, proxy))
    }

    fn test_systemd() -> zbus::Result<()> {
        let (_conn, proxy) = setup()?;
        let result = proxy.get_default_target()?;
        assert_eq!(result, "default.target");
        Ok((()))
    }

    #[test]
    fn test_systemd_unit() -> zbus::Result<()> {
        let (_conn, proxy) = setup()?;
        let result = proxy.get_unit("default.target")?;
        assert_eq!(result.id()?, "default.target");
        Ok(())
    }

}
