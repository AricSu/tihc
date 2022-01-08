// CheckOptions contains the options for check command
pub struct CheckOptions {
	User         :string, // username to login to the SSH server
	IdentityFile :string, // path to the private key file
	UsePassword  :bool,   // use password instead of identity file for ssh connection
	Opr          :*operator.CheckOptions,
	ApplyFix     :bool, // try to apply fixes of failed checks
	ExistCluster :bool, // check an exist cluster
}


impl CheckOptions{
	
}

// checkSystemInfo performs series of checks and tests of the deploy server
pub fn checkSystemInfo(s, p *tui.SSHConnectionProps, topo *spec.Specification, gOpt *operator.Options, opt *CheckOptions) error {
	var (
		collectTasks  []*task.StepDisplay
		checkSysTasks []*task.StepDisplay
		cleanTasks    []*task.StepDisplay
		applyFixTasks []*task.StepDisplay
		downloadTasks []*task.StepDisplay
	)
	insightVer := spec.TiDBComponentVersion(spec.ComponentCheckCollector, "")

	uniqueHosts := map[string]int{}             // host -> ssh-port
	uniqueArchList := make(map[string]struct{}) // map["os-arch"]{}

	roleFilter := set.NewStringSet(gOpt.Roles...)
	nodeFilter := set.NewStringSet(gOpt.Nodes...)
	components := topo.ComponentsByUpdateOrder()
	components = operator.FilterComponent(components, roleFilter)

	for _, comp := range components {
		instances := operator.FilterInstance(comp.Instances(), nodeFilter)
		if len(instances) < 1 {
			continue
		}

		for _, inst := range instances {
			archKey := fmt.Sprintf("%s-%s", inst.OS(), inst.Arch())
			if _, found := uniqueArchList[archKey]; !found {
				uniqueArchList[archKey] = struct{}{}
				t0 := task.NewBuilder().
					Download(
						spec.ComponentCheckCollector,
						inst.OS(),
						inst.Arch(),
						insightVer,
					).
					BuildAsStep(fmt.Sprintf("  - Downloading check tools for %s/%s", inst.OS(), inst.Arch()))
				downloadTasks = append(downloadTasks, t0)
			}

			t1 := task.NewBuilder()
			// checks that applies to each instance
			if opt.ExistCluster {
				t1 = t1.CheckSys(
					inst.GetHost(),
					inst.DeployDir(),
					task.CheckTypePermission,
					topo,
					opt.Opr,
				)
			}
			// if the data dir set in topology is relative, and the home dir of deploy user
			// and the user run the check command is on different partitions, the disk detection
			// may be using incorrect partition for validations.
			for _, dataDir := range spec.MultiDirAbs(opt.User, inst.DataDir()) {
				// build checking tasks
				t1 = t1.
					CheckSys(
						inst.GetHost(),
						dataDir,
						task.CheckTypeFIO,
						topo,
						opt.Opr,
					)
				if opt.ExistCluster {
					t1 = t1.CheckSys(
						inst.GetHost(),
						dataDir,
						task.CheckTypePermission,
						topo,
						opt.Opr,
					)
				}
			}

			// checks that applies to each host
			if _, found := uniqueHosts[inst.GetHost()]; !found {
				uniqueHosts[inst.GetHost()] = inst.GetSSHPort()
				// build system info collecting tasks
				t2 := task.NewBuilder().
					RootSSH(
						inst.GetHost(),
						inst.GetSSHPort(),
						opt.User,
						s.Password,
						s.IdentityFile,
						s.IdentityFilePassphrase,
						gOpt.SSHTimeout,
						gOpt.OptTimeout,
						gOpt.SSHProxyHost,
						gOpt.SSHProxyPort,
						gOpt.SSHProxyUser,
						p.Password,
						p.IdentityFile,
						p.IdentityFilePassphrase,
						gOpt.SSHProxyTimeout,
						gOpt.SSHType,
						topo.GlobalOptions.SSHType,
					).
					Mkdir(opt.User, inst.GetHost(), filepath.Join(task.CheckToolsPathDir, "bin")).
					CopyComponent(
						spec.ComponentCheckCollector,
						inst.OS(),
						inst.Arch(),
						insightVer,
						"", // use default srcPath
						inst.GetHost(),
						task.CheckToolsPathDir,
					).
					Shell(
						inst.GetHost(),
						filepath.Join(task.CheckToolsPathDir, "bin", "insight"),
						"",
						false,
					).
					BuildAsStep(fmt.Sprintf("  - Getting system info of %s:%d", inst.GetHost(), inst.GetSSHPort()))
				collectTasks = append(collectTasks, t2)

				// build checking tasks
				t1 = t1.
					// check for general system info
					CheckSys(
						inst.GetHost(),
						"",
						task.CheckTypeSystemInfo,
						topo,
						opt.Opr,
					).
					CheckSys(
						inst.GetHost(),
						"",
						task.CheckTypePartitions,
						topo,
						opt.Opr,
					).
					// check for listening port
					Shell(
						inst.GetHost(),
						"ss -lnt",
						"",
						false,
					).
					CheckSys(
						inst.GetHost(),
						"",
						task.CheckTypePort,
						topo,
						opt.Opr,
					).
					// check for system limits
					Shell(
						inst.GetHost(),
						"cat /etc/security/limits.conf",
						"",
						false,
					).
					CheckSys(
						inst.GetHost(),
						"",
						task.CheckTypeSystemLimits,
						topo,
						opt.Opr,
					).
					// check for kernel params
					Shell(
						inst.GetHost(),
						"sysctl -a",
						"",
						true,
					).
					CheckSys(
						inst.GetHost(),
						"",
						task.CheckTypeSystemConfig,
						topo,
						opt.Opr,
					).
					// check for needed system service
					CheckSys(
						inst.GetHost(),
						"",
						task.CheckTypeService,
						topo,
						opt.Opr,
					).
					// check for needed packages
					CheckSys(
						inst.GetHost(),
						"",
						task.CheckTypePackage,
						topo,
						opt.Opr,
					)
			}

			checkSysTasks = append(
				checkSysTasks,
				t1.BuildAsStep(fmt.Sprintf("  - Checking node %s", inst.GetHost())),
			)

			t3 := task.NewBuilder().
				RootSSH(
					inst.GetHost(),
					inst.GetSSHPort(),
					opt.User,
					s.Password,
					s.IdentityFile,
					s.IdentityFilePassphrase,
					gOpt.SSHTimeout,
					gOpt.OptTimeout,
					gOpt.SSHProxyHost,
					gOpt.SSHProxyPort,
					gOpt.SSHProxyUser,
					p.Password,
					p.IdentityFile,
					p.IdentityFilePassphrase,
					gOpt.SSHProxyTimeout,
					gOpt.SSHType,
					topo.GlobalOptions.SSHType,
				).
				Rmdir(inst.GetHost(), task.CheckToolsPathDir).
				BuildAsStep(fmt.Sprintf("  - Cleanup check files on %s:%d", inst.GetHost(), inst.GetSSHPort()))
			cleanTasks = append(cleanTasks, t3)
		}
	}

	t := task.NewBuilder().
		ParallelStep("+ Download necessary tools", false, downloadTasks...).
		ParallelStep("+ Collect basic system information", false, collectTasks...).
		ParallelStep("+ Check system requirements", false, checkSysTasks...).
		ParallelStep("+ Cleanup check files", false, cleanTasks...).
		Build()

	ctx := ctxt.New(context.Background(), gOpt.Concurrency)
	if err := t.Execute(ctx); err != nil {
		if errorx.Cast(err) != nil {
			// FIXME: Map possible task errors and give suggestions.
			return err
		}
		return perrs.Trace(err)
	}

	// FIXME: add fix result to output
	checkResultTable := [][]string{
		// Header
		{"Node", "Check", "Result", "Message"},
	}
	checkResults := make([]HostCheckResult, 0)
	for host := range uniqueHosts {
		tf := task.NewBuilder().
			RootSSH(
				host,
				uniqueHosts[host],
				opt.User,
				s.Password,
				s.IdentityFile,
				s.IdentityFilePassphrase,
				gOpt.SSHTimeout,
				gOpt.OptTimeout,
				gOpt.SSHProxyHost,
				gOpt.SSHProxyPort,
				gOpt.SSHProxyUser,
				p.Password,
				p.IdentityFile,
				p.IdentityFilePassphrase,
				gOpt.SSHProxyTimeout,
				gOpt.SSHType,
				topo.GlobalOptions.SSHType,
			)
		res, err := handleCheckResults(ctx, host, opt, tf)
		if err != nil {
			continue
		}
		checkResults = append(checkResults, res...)
		applyFixTasks = append(applyFixTasks, tf.BuildAsStep(fmt.Sprintf("  - Applying changes on %s", host)))
	}
	resLines := formatHostCheckResults(checkResults)
	checkResultTable = append(checkResultTable, resLines...)

	// print check results *before* trying to applying checks
	// FIXME: add fix result to output, and display the table after fixing
	tui.PrintTable(checkResultTable, true)

	if opt.ApplyFix {
		tc := task.NewBuilder().
			ParallelStep("+ Try to apply changes to fix failed checks", false, applyFixTasks...).
			Build()
		if err := tc.Execute(ctx); err != nil {
			if errorx.Cast(err) != nil {
				// FIXME: Map possible task errors and give suggestions.
				return err
			}
			return perrs.Trace(err)
		}
	}

	return nil
}

// HostCheckResult represents the check result of each node
type HostCheckResult struct {
	Node    string `json:"node"`
	Name    string `json:"name"`
	Status  string `json:"status"`
	Message string `json:"message"`
}

// handleCheckResults parses the result of checks
func handleCheckResults(ctx context.Context, host string, opt *CheckOptions, t *task.Builder) ([]HostCheckResult, error) {
	rr, _ := ctxt.GetInner(ctx).GetCheckResults(host)
	if len(rr) < 1 {
		return nil, fmt.Errorf("no check results found for %s", host)
	}
	results := []*operator.CheckResult{}
	for _, r := range rr {
		results = append(results, r.(*operator.CheckResult))
	}

	items := make([]HostCheckResult, 0)
	// log.Infof("Check results of %s: (only errors and important info are displayed)", color.HiCyanString(host))
	for _, r := range results {
		var item HostCheckResult
		if r.Err != nil {
			if r.IsWarning() {
				item = HostCheckResult{Node: host, Name: r.Name, Status: "Warn", Message: r.Error()}
			} else {
				item = HostCheckResult{Node: host, Name: r.Name, Status: "Fail", Message: r.Error()}
			}
			if !opt.ApplyFix {
				items = append(items, item)
				continue
			}
			msg, err := fixFailedChecks(host, r, t)
			if err != nil {
				log.Debugf("%s: fail to apply fix to %s (%s)", host, r.Name, err)
			}
			if msg != "" {
				// show auto fixing info
				item.Message = msg
			}
		} else if r.Msg != "" {
			item = HostCheckResult{Node: host, Name: r.Name, Status: "Pass", Message: r.Msg}
		}

		// show errors and messages only, ignore empty lines
		// if len(line) > 0 {
		if len(item.Node) > 0 {
			items = append(items, item)
		}
	}

	return items, nil
}

func formatHostCheckResults(results []HostCheckResult) [][]string {
	lines := make([][]string, 0)
	for _, r := range results {
		var coloredStatus string
		switch r.Status {
		case "Warn":
			coloredStatus = color.YellowString(r.Status)
		case "Fail":
			coloredStatus = color.HiRedString(r.Status)
		default:
			coloredStatus = color.GreenString(r.Status)
		}
		line := []string{r.Node, r.Name, coloredStatus, r.Message}
		lines = append(lines, line)
	}
	return lines
}
