/* vim: set et ts=4 sw=4: */
/* avahi.rs
 *
 * Copyright (C) 2017 Pelagicore AB.
 * Copyright (C) 2017 Zeeshan Ali.
 *
 * GPSShare is free software; you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * GPSShare is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License along
 * with GPSShare; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 *
 * Author: Zeeshan Ali <zeeshanak@gnome.org>
 */

use dbus;

pub struct Server<'a> {
    bus_name: String,
    path: dbus::Path<'a>,
    connection: dbus::Connection,
}
impl <'a> Server<'a> {
    pub fn new<P>(dbus_name: &str, path: P, dbus_connection: dbus::Connection)
     -> Self where P: Into<dbus::Path<'a>> {
        Server{bus_name: dbus_name.to_string(),
               path: path.into(),
               connection: dbus_connection,
              }
    }
    pub fn entry_group_new(&self) -> Result<dbus::Path, dbus::Error> {
        let message =
            dbus::Message::new_method_call(&self.bus_name,
                                           self.path.clone(),
                                           "org.freedesktop.Avahi.Server",
                                           ::dbus_macros::to_camel("entry_group_new")).unwrap();
        let response = try!(self.connection.send_with_reply_and_block(message, 2000));
        response.get1().ok_or(dbus::Error::from(dbus::tree::MethodErr::no_arg()))
    }
}
pub struct EntryGroup<'a> {
    bus_name: String,
    path: dbus::Path<'a>,
    connection: dbus::Connection,
}
impl <'a> EntryGroup<'a> {
    pub fn new<P>(dbus_name: &str, path: P, dbus_connection: dbus::Connection)
     -> Self where P: Into<dbus::Path<'a>> {
        EntryGroup{bus_name: dbus_name.to_string(),
                   path: path.into(),
                   connection: dbus_connection,
                    }
    }
    pub fn add_service(&self, ifindex: i32, protocol: i32, flags: u32,
                       name: &str, service_type: &str, domain: &str,
                       host: &str, port: u16, text: &str)
     -> Result<(), dbus::Error> {
        let message =
            dbus::Message::new_method_call(&self.bus_name,
                                           self.path.clone(),
                                           "org.freedesktop.Avahi.EntryGroup",
                                           ::dbus_macros::to_camel("add_service")).unwrap();
        let message = message.append1(ifindex);
        let message = message.append1(protocol);
        let message = message.append1(flags);
        let message = message.append1(name);
        let message = message.append1(service_type);
        let message = message.append1(domain);
        let message = message.append1(host);
        let message = message.append1(port);
        let message = message.append1(text);
        self.connection.send(message).ok();
        Ok(())
    }
    pub fn commit(&self) -> Result<(), dbus::Error> {
        let message =
            dbus::Message::new_method_call(&self.bus_name,
                                           self.path.clone(),
                                           "org.freedesktop.Avahi.EntryGroup",
                                           ::dbus_macros::to_camel("commit")).unwrap();
        self.connection.send(message).ok();
        Ok(())
    }
}

pub struct Avahi<'a> {
    server: Server<'a>,
    connection: dbus::Connection,
}

impl<'a> Avahi<'a> {
    pub fn new() -> Self {
        let connection: dbus::Connection = dbus::Connection::get_private(dbus::BusType::System).unwrap();
        let server: Server = Server::new("org.freedesktop.Avahi", "/", connection);
   
        Avahi { server: server, connection: connection }
    }

    pub fn publish(&self, port: u16) -> Result<(),dbus::Error> {
        // FIXME: Make this async when it's possible
        let group_path = self.server.entry_group_new()?;
        println!("group: {}", group_path);

        let group = EntryGroup::new("org.freedesktop.Avahi", group_path, self.connection);
        group.add_service(-1, -1, 0, "gps-share", "_nmea-0183._tcp", "", "", port, "")?;
        group.commit()?;

        Ok(())
    }
}
