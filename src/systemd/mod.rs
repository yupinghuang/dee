use zbus::{zvariant::ObjectPath, Connection, Result, dbus_proxy};

#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Unit",
    default_service = "org.freedesktop.systemd1",
)]
trait SystemdUnit {
    #[dbus_proxy(property)]
    fn Id(&self) -> Result<String>;
}

#[dbus_proxy(
    name = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
trait SystemdManager {
    fn getDefaultTarget(&self) -> Result<String>;
    #[dbus_proxy(object = "SystemdUnit")]
    fn getUnit<'a>(&self, name: &str);
}

pub async fn get_systemd_status() -> Result<String> {
    let connection = Connection::session().await?;
    let proxy = SystemdManagerProxy::new(&connection).await?;
    let reply = proxy.getDefaultTarget().await?;
    Ok(reply)
}
#[cfg(test)]
mod tests {
    use rocket::tokio;
    use super::*;

    #[tokio::test]
    async fn test_systemd() {
        let result = get_systemd_status().await;
        assert_eq!(result.unwrap(), "default.target");
    }
}