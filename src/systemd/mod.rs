use serde::{Deserialize, Serialize};
use zbus;
use zbus::{
    dbus_proxy,
    zvariant::ObjectPath,
    Connection
};

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

pub async fn elevate_service(conn: &Connection, name: &str) -> zbus::Result<()> {
    let proxy = SystemdManagerProxy::new(conn).await?;
    let unit = proxy.get_unit(name).await?;
    let unit_id = unit.id().await?;
    let properties = vec!["Slice".to_string()];
    proxy.SetUnitProperties(&unit_id, true, properties).await?;
    Ok(())
}

//pub async fn elevate_slice(conn: &Connection, name: &str) -> zbus::Result<()> {
//    let proxy = SystemdManagerProxy::new(conn).await?;
//    let slice = proxy.get_unit(name).await?;
//    let slice_id = slice.slice().await?;
//    let properties = vec!["Slice".to_string()];
//    proxy.SetUnitProperties(&slice_id, true, properties).await?;
//    Ok(())
//}

#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Unit",
    default_service = "org.freedesktop.systemd1"
)]
trait SystemdUnit {
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;
}

#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Slice",
    default_service = "org.freedesktop.systemd1",
)]
trait SystemdSlice {

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
    default_path = "/org/freedesktop/systemd1"
)]
trait SystemdManager {
    #[dbus_proxy(property)]
    fn version(&self) -> zbus::Result<String>;

    #[dbus_proxy(signal)]
    fn UnitNew(&self, name: &str, unit: ObjectPath<'_>) -> zbus::Result<()>;

    fn SetUnitProperties(
        &self,
        name: &str,
        runtime: bool,
        properties: Vec<String>,
    ) -> zbus::Result<()>;

    fn get_default_target(&self) -> zbus::Result<String>;

    #[dbus_proxy(object = "SystemdUnit")]
    fn get_unit(&self, name: &str);
}

impl<'a> SystemdSliceProxy<'a> {
    async fn from<'b>(u: &'b SystemdUnitProxy<'b>) -> zbus::Result<SystemdSliceProxy<'b>> {
        let p = u.path().clone();
        let s = SystemdSliceProxy::builder(u.connection())
            .path(p)?
            .build()
            .await?;
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::tokio;
    use zbus::Connection;

    async fn setup<'a>() -> zbus::Result<(Connection, SystemdManagerProxy<'a>)> {
        let conn = Connection::system().await?;
        let proxy = SystemdManagerProxy::new(&conn).await?;
        Ok((conn, proxy))
    }

    #[tokio::test]
    async fn test_systemd() {
        let (_conn, proxy) = setup().await.unwrap();
        let result = proxy.get_default_target().await.unwrap();
        assert_eq!(result, "default.target");
    }

    #[tokio::test]
    async fn test_systemd_unit() {
        let (_conn, proxy) = setup().await.unwrap();
        let result = proxy.get_unit("default.target").await.unwrap();
        assert_eq!(result.id().await.unwrap(), "default.target");
    }

    #[tokio::test]
    async fn convert_unit_to_slice() {
        let (_conn, proxy) = setup().await.unwrap();
        let unit = proxy.get_unit("user.slice").await.unwrap();
        let slice = SystemdSliceProxy::from(&unit).await.unwrap();
        assert_eq!(slice.path(), unit.path());
        assert_eq!(slice.path().to_string(), String::from("/org/freedesktop/systemd1/unit/user_2eslice"));
    }
}
