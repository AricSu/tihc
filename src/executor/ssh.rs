use ssh2::Session;
use std::net::TcpStream;

#[derive(Clone)]
pub struct SSHConfig {
    pub host: String,     // hostname of the SSH server
    pub port: i64,        // port of the SSH server
    pub user: String,     // username to login to the SSH server
    pub password: String, // password of the user
}

impl SSHConfig {
    pub fn new_auth_user(host: String, port: i64, user: String, password: String) -> Self {
        SSHConfig {
            host: host,
            port: port,
            user: user,
            password: password,
        }
    }

    pub fn new_ssession(&self) -> Session {
        let tcp = TcpStream::connect(format!("{}:{}", self.host, self.port)).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        let _ = sess.userauth_password(self.user.as_str(), self.password.as_str());
        assert!(sess.authenticated());
        return sess;
    }
}

// the interface to handle ssh action for all nodes
pub struct ClusterSSHHandle {
    pub all_config: Vec<SSHConfig>,
    pub all_handler: Vec<Session>,
}

impl ClusterSSHHandle {
    // use to get session for every ssh node
    pub fn new(ssh_list: &Vec<SSHConfig>) -> Self {
        let mut inner_all_handler = vec![];
        for node_config in ssh_list {
            inner_all_handler.append(&mut vec![node_config.new_ssession()]);
        }
        return ClusterSSHHandle {
            all_config: ssh_list.to_vec(),
            all_handler: inner_all_handler,
        };
    }
}
