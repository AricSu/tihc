use ssh2::Session;
use std::net::TcpStream;

#[derive(Clone)]
pub struct SSHConfig {
    pub host: String,       // hostname of the SSH server
    pub port: u64,          // port of the SSH server
    pub user: String,       // username to login to the SSH server
    pub password: String,   // password of the user
    pub key_file: String,   // path to the private key file
    pub passphrase: String, // passphrase of the private key file
    pub timeout: u64, // Timeout is the maximum amount of time for the TCP connection to establish.
    pub exe_timeout: u64, // ExeTimeout is the maximum amount of time for the command to finish
}

impl SSHConfig {
    pub fn new() -> Self {
        SSHConfig {
            host: "139.155.15.210".to_string(),
            port: 7006,
            user: "tidb".to_string(),
            password: "tidb".to_string(),
            key_file: "".to_string(),
            passphrase: "".to_string(),
            timeout: 1000,
            exe_timeout: 1000,
        }
    }
    pub fn new_ssession(&self) -> Session {
        let tcp = TcpStream::connect(format!("{}:{}", self.host, self.port)).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        sess.userauth_password(self.user.as_str(), self.password.as_str());
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

// // SSHType represent the type of the chanel used by ssh
// type SSHType = string;

// var (
// 	errNS = errorx.NewNamespace("executor")

// 	// SSHTypeBuiltin is the type of easy ssh executor
// 	SSHTypeBuiltin SSHType = "builtin"

// 	// SSHTypeSystem is the type of host ssh client
// 	SSHTypeSystem SSHType = "system"

// 	// SSHTypeNone is the type of local executor (no ssh will be used)
// 	SSHTypeNone SSHType = "none"

// 	execute_default_timeout = time.Second * 60

// 	// This command will be execute once the NativeSSHExecutor is created.
// 	// It's used to predict if the connection can establish success in the future.
// 	// Its main purpose is to avoid sshpass hang when user speficied a wrong prompt.
// 	connectionTestCommand = "echo connection test, if killed, check the password prompt"

// 	// SSH authorized_keys file
// 	defaultSSHAuthorizedKeys = "~/.ssh/authorized_keys"
// )

// // New create a new Executor
// func New(etype SSHType, sudo bool, c SSHConfig) -> (ctxt.Executor, error) {
// 	if etype == "" {
// 		etype = SSHTypeBuiltin
// 	}

// 	// Used in integration testing, to check if native ssh client is really used when it need to be.
// 	failpoint.Inject("assertNativeSSH", func() {
// 		// XXX: We call system executor 'native' by mistake in commit f1142b1
// 		// this should be fixed after we remove --native-ssh flag
// 		if etype != SSHTypeSystem {
// 			msg := fmt.Sprintf(
// 				"native ssh client should be used in this case, os.Args: %s, %s = %s",
// 				os.Args, localdata.EnvNameNativeSSHClient, os.Getenv(localdata.EnvNameNativeSSHClient),
// 			)
// 			panic(msg)
// 		}
// 	})

// 	// set default values
// 	if c.Port <= 0 {
// 		c.Port = 22
// 	}

// 	if c.Timeout == 0 {
// 		c.Timeout = time.Second * 5 // default timeout is 5 sec
// 	}

// 	let executor = ctxt.Executor;
// 	match etype {
// 	case SSHTypeBuiltin:
// 		e := &EasySSHExecutor{
// 			Locale: "C",
// 			Sudo:   sudo,
// 		}
// 		e.initialize(c)
// 		executor = e
// 	case SSHTypeSystem:
// 		e := &NativeSSHExecutor{
// 			Config: &c,
// 			Locale: "C",
// 			Sudo:   sudo,
// 		}
// 		if c.Password != "" || (c.KeyFile != "" && c.Passphrase != "") {
// 			_, _, e.ConnectionTestResult = e.Execute(context.Background(), connectionTestCommand, false, executeDefaultTimeout)
// 		}
// 		executor = e
// 	case SSHTypeNone:
// 		if err := checkLocalIP(c.Host); err != nil {
// 			return nil, err
// 		}
// 		e := &Local{
// 			Config: &c,
// 			Sudo:   sudo,
// 			Locale: "C",
// 		}
// 		executor = e
// 	default:
// 		return nil, errors.Errorf("unregistered executor: %s", etype)
// 	}

// 	return &CheckPointExecutor{executor, &c}, nil
// }

// func checkLocalIP(ip string) error {
// 	ifaces, err := net.Interfaces()
// 	if err != nil {
// 		return errors.AddStack(err)
// 	}

// 	foundIps := []string{}
// 	for _, i := range ifaces {
// 		addrs, err := i.Addrs()
// 		if err != nil {
// 			continue
// 		}

// 		for _, addr := range addrs {
// 			switch v := addr.(type) {
// 			case *net.IPNet:
// 				if ip == v.IP.String() {
// 					return nil
// 				}
// 				foundIps = append(foundIps, v.IP.String())
// 			case *net.IPAddr:
// 				if ip == v.IP.String() {
// 					return nil
// 				}
// 				foundIps = append(foundIps, v.IP.String())
// 			}
// 		}
// 	}

// 	return fmt.Errorf("address %s not found in all interfaces, found ips: %s", ip, strings.Join(foundIps, ","))
// }

// // FindSSHAuthorizedKeysFile finds the correct path of SSH authorized keys file
// func FindSSHAuthorizedKeysFile(ctx context.Context, exec ctxt.Executor) string {
// 	// detect if custom path of authorized keys file is set
// 	// NOTE: we do not yet support:
// 	//   - custom config for user (~/.ssh/config)
// 	//   - sshd started with custom config (other than /etc/ssh/sshd_config)
// 	//   - ssh server implementations other than OpenSSH (such as dropbear)
// 	sshAuthorizedKeys := defaultSSHAuthorizedKeys
// 	cmd := "grep -Ev '^\\s*#|^\\s*$' /etc/ssh/sshd_config"
// 	stdout, _, _ := exec.Execute(ctx, cmd, true) // error ignored as we have default value
// 	for _, line := range strings.Split(string(stdout), "\n") {
// 		if !strings.Contains(line, "AuthorizedKeysFile") {
// 			continue
// 		}
// 		fields := strings.Fields(line)
// 		if len(fields) >= 2 {
// 			sshAuthorizedKeys = fields[1]
// 			break
// 		}
// 	}

// 	if !strings.HasPrefix(sshAuthorizedKeys, "/") && !strings.HasPrefix(sshAuthorizedKeys, "~") {
// 		sshAuthorizedKeys = fmt.Sprintf("~/%s", sshAuthorizedKeys)
// 	}
// 	return sshAuthorizedKeys
// }

// SSHConfig is the configuration needed to establish SSH connection.
