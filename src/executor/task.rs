use operator::*;
use anyhow::Result;

// the check types
const	CHECK_TYPE_SYSTEM_INFO:str   = "insight";
const	CHECK_TYPE_SYSTEM_LIMITS:str = "limits";
const	CheckTypeSystemConfig:str = "system";
const	CheckTypePort :str        = "port";
const	CheckTypeService :str     = "service";
const	CheckTypePackage :str     = "package";
const	CheckTypePartitions :str  = "partitions";
const	CheckTypeFIO       :str   = "fio";
const	CheckTypePermission  :str = "permission";


// CheckSys performs checks of system information
pub struct CheckSys  {
	host     :String,
	topo     :String,
	opt      :operator::CheckOptions,
	check    :String, // check type name
	checkDir :String
}

fn storeResults(host: String, results: &[operator::CheckResult]) {
	rr = []interface{}{};
	for _, r = range results {
		rr = append(rr, r)
	}
	ctxt.GetInner(ctx).SetCheckResults(host, rr)
}

// Execute implements the Task interface
impl Task for CheckSys{
    fn Execute(&self) -> Result {
        match self.check {
        CheckTypeSystemInfo => storeResults(self.host, operator::CheckSystemInfo(self.opt, stdout),
        CheckTypeSystemLimits => storeResults(self.host, operator.CheckSysLimits(self.opt, self.topo.GlobalOptions.User, stdout)),
        CheckTypeSystemConfig=> {
            results = operator.CheckKernelParameters(self.opt, stdout)
            e, ok = ctxt.GetInner(ctx).GetExecutor(self.host)
            if !ok {
                return ErrNoExecutor
            }
            results = append(
                results,
                operator.CheckSELinux( e),
                operator.CheckTHP( e),
            )
            storeResults( self.host, results)
        }
            
        CheckTypePort => storeResults( self.host, operator.CheckListeningPort(self.opt, self.host, self.topo, stdout))
        CheckTypeService =>{
            e, ok = ctxt.GetInner(ctx).GetExecutor(self.host)
            if !ok {
                return ErrNoExecutor
            }
            var results []*operator.CheckResult
    
            // check services
            results = append(
                results,
                operator.CheckServices( e, self.host, "irqbalance", false),
                // FIXME: set firewalld rules in deploy, and not disabling it anymore
                operator.CheckServices( e, self.host, "firewalld", true),
            )
            storeResults( self.host, results)
        }
            
        CheckTypePackage => {
            // check if a command present, and if a package installed
            e, ok = ctxt.GetInner(ctx).GetExecutor(self.host)
            if !ok {
                return ErrNoExecutor
            }
            var results []*operator.CheckResult

            // check if numactl is installed
            stdout, stderr, err = e.Execute( "numactl --show", false)
            if err != nil || len(stderr) > 0 {
                results = append(results, &operator.CheckResult{
                    Name: operator.CheckNameCommand,
                    Err:  fmt.Errorf("numactl not usable, %s", Strings.Trim(String(stderr), "\n")),
                    Msg:  "numactl is not installed properly",
                })
            } else {
                results = append(results, &operator.CheckResult{
                    Name: operator.CheckNameCommand,
                    Msg:  "numactl: " + Strings.Split(String(stdout), "\n")[0],
                })
            }

            // check if JRE is available for TiSpark
            results = append(results, operator.CheckJRE( e, self.host, self.topo)...)

            storeResults( self.host, results)
        }
        // check partition mount options for data_dir
        CheckTypePartitions => storeResults( self.host, operator.CheckPartitions(self.opt, self.host, self.topo, stdout))
        CheckTypeFIO => {
            if !self.opt.EnableDisk || self.checkDir == "" {
                break
            }
    
            rr, rw, lat, err = self.runFIO(ctx)
            if err != nil {
                return err
            }
    
            storeResults( self.host, operator.CheckFIOResult(rr, rw, lat))
        }
        CheckTypePermission => {
            e, ok = ctxt.GetInner(ctx).GetExecutor(self.host)
            if !ok {
                return ErrNoExecutor
            }
            storeResults( self.host, operator.CheckDirPermission( e, self.topo.GlobalOptions.User, self.checkDir))
        }
    }

    // Rollback implements the Task interface
    func (c *CheckSys) Rollback(ctx context.Context) error {
	    return ErrUnsupportedRollback
    }

    // String implements the fmt.Stringer interface
    func (c *CheckSys) String() String {
	    return fmt.Sprintf("CheckSys: host=%s type=%s", self.host, self.check)
    }
}



