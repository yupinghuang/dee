
use zbus;
use zbus::{
    dbus_proxy,
    Proxy,
    zvariant::ObjectPath,
};

type Properties = Vec<(String, String)>;

const NOT_SET: u64 = 18446744073709551615;

#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Unit",
    default_service = "org.freedesktop.systemd1",
    gen_blocking=false,
    assume_defaults=false
)]
trait Systemd1Unit {
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn transient(&self) -> zbus::Result<bool>;

    fn set_properties(&self, runtime: bool, properties: Properties) -> zbus::Result<()>;
}

#[dbus_proxy(
    name = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1",
    gen_blocking=false,
    assume_defaults=true
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

struct SliceUnitProxy<'a> {
    unit_proxy: Systemd1UnitProxy<'a>,
    proxy: Proxy<'a>
}


impl<'a> SliceUnitProxy<'a> {
    fn unit_proxy(&self) -> &Systemd1UnitProxy<'a> {
        &self.unit_proxy
    }

    async fn from_name(manager: &Systemd1ManagerProxy<'a>, name: &str) -> zbus::Result<Self> {
        let unit_proxy = manager.get_unit(name).await?;
        Self::tryfrom(unit_proxy).await
    }

    async fn tryfrom(unit_proxy: Systemd1UnitProxy<'a>) -> zbus::Result<Self> {
        let proxy = Proxy::new(
            unit_proxy.connection(),
            "org.freedesktop.systemd1",
            unit_proxy.path().to_owned(),
            "org.freedesktop.systemd1.Slice"
        ).await?;
        Ok(Self {
            unit_proxy,
            proxy
        })
    }

    async fn slice(&self) -> zbus::Result<String> {
        self.proxy.get_property("Slice").await
    }

    async fn control_group(&self) -> zbus::Result<String> {
        self.proxy.get_property("ControlGroup").await
    }

    async fn cpu_accounting(&self) -> zbus::Result<bool> {
        self.proxy.get_property("CPUAccounting").await
    }

    async fn cpu_shares(&self) -> zbus::Result<u64> {
        self.proxy.get_property("CPUShares").await
    }

    async fn block_io_accounting(&self) -> zbus::Result<bool> {
        self.proxy.get_property("BlockIOAccounting").await
    }

    async fn block_io_weight(&self) -> zbus::Result<u64> {
        self.proxy.get_property("BlockIOWeight").await
    }

    async fn block_io_device_weight(&self) -> zbus::Result<Vec<(String, u64)>> {
        self.proxy.get_property("BlockIODeviceWeight").await
    }

    async fn block_io_read_bandwidth(&self) -> zbus::Result<Vec<(String, u64)>> {
        self.proxy.get_property("BlockIOReadBandwidth").await
    }

    async fn block_io_write_bandwidth(&self) -> zbus::Result<Vec<(String, u64)>> {
        self.proxy.get_property("BlockIOWriteBandwidth").await
    }

    async fn memory_accounting(&self) -> zbus::Result<bool> {
        self.proxy.get_property("MemoryAccounting").await
    }

    async fn memory_limit(&self) -> zbus::Result<u64> {
        self.proxy.get_property("MemoryLimit").await
    }

    async fn device_policy(&self) -> zbus::Result<String> {
        self.proxy.get_property("DevicePolicy").await
    }

    async fn device_allow(&self) -> zbus::Result<Vec<(String, String)>> {
        self.proxy.get_property("DeviceAllow").await
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rocket::tokio;
    use zbus::Connection;

    async fn setup<'a>() -> zbus::Result<(Connection, Systemd1ManagerProxy<'a>)> {
        let conn = Connection::system().await?;
        let proxy = Systemd1ManagerProxy::new(&conn).await?;
        Ok((conn, proxy))
    }

    #[tokio::test]
    async fn systemd_get() -> zbus::Result<()> {
        let (_conn, proxy) = setup().await?;
        let result = proxy.get_default_target().await?;
        assert_eq!(result, "default.target");
        Ok(())
    }

    #[tokio::test]
    async fn slice_unit_proxy() -> zbus::Result<()> {
        let (_conn, proxy) = setup().await?;
        let su = proxy.get_unit("user.slice").await?;
        let id = su.id().await?;
        let slice = SliceUnitProxy::tryfrom(su).await?;
        let id2 = slice.unit_proxy().id();
        let cgroup = slice.control_group();
        let cpu_shares = slice.cpu_shares();

        assert_eq!(id, "user.slice");
        assert_eq!(id, id2.await?);
        assert_eq!(cgroup.await?, "/user.slice");
        assert_eq!(cpu_shares.await?, NOT_SET);
        Ok(())
    }

}
