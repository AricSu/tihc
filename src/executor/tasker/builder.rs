// Task represents a operation while TiUP execution
trait Task{
	fn to_string() -> String;
	fn execute() -> String;
    fn rollback() -> String;
}


// Serial will execute a bundle of task in serialized way
pub struct Serial{
	hideDetailDisplay: bool,
	inner: &[Task]
}
// Parallel will execute a bundle of task in parallelism way
// pub struct Parallel{
// 	hideDetailDisplay: bool,
// 	inner: &[Task]
// }

// Builder is used to build TiUP task
struct Builder {
	tasks: &[Task]
}

// ClusterSSH init all UserSSH need for the cluster.
impl Builder{
    fn ClusterSSH(
    	topo spec.Topology,
    	deployUser string, sshTimeout, exeTimeout uint64,
    	proxyHost string, proxyPort int, proxyUser, proxyPassword, proxyKeyFile, proxyPassphrase string, proxySSHTimeout uint64,
    	sshType, defaultSSHType executor.SSHType,
    ) -> Self {
    	if sshType == "" {
    		sshType = defaultSSHType
    	}
    	var tasks []Task
    	topo.IterInstance(func(inst spec.Instance) {
    		tasks = append(tasks, &UserSSH{
    			host:            inst.GetHost(),
    			port:            inst.GetSSHPort(),
    			deployUser:      deployUser,
    			timeout:         sshTimeout,
    			exeTimeout:      exeTimeout,
    			proxyHost:       proxyHost,
    			proxyPort:       proxyPort,
    			proxyUser:       proxyUser,
    			proxyPassword:   proxyPassword,
    			proxyKeyFile:    proxyKeyFile,
    			proxyPassphrase: proxyPassphrase,
    			proxyTimeout:    proxySSHTimeout,
    			sshType:         sshType,
    		})
    	})

    	b.tasks = append(b.tasks, &Parallel{inner: tasks})

    	return b
    }
}


impl Task for Serial{
    // Execute implements the Task interface
    fn execute(self) -> error {
        // for serial_task in self.inner {
        //     ctxt.GetInner(ctx).Ev.PublishTaskBegin(t);
        //     err := t.Execute(ctx);
        //     ctxt.GetInner(ctx).Ev.PublishTaskFinish(t, err);
        //     if err != nil && !s.ignoreError {
        //         return err
        //     }
        // }
        return nil
    }


    // Rollback implements the Task interface
    fn Rollback(self) -> error {
        // Rollback in reverse order
        for i := len(s.inner) - 1; i >= 0; i-- {
            err := s.inner[i].Rollback(ctx)
            if err != nil {
                return err
            }
        }
        return nil
    }

    // String implements the fmt.Stringer interface
    fn to_string(self) -> String {
        let ss = &[String];
        for t in self.inner {
            ss.append(ss, t.to_string())
        }
        return strings.Join(ss, "\n")
    }
}

// impl Task for Parallel{
//     // Execute implements the Task interface
// func (pt *Parallel) Execute(ctx context.Context) error {
// 	var firstError error
// 	var mu sync.Mutex
// 	wg := sync.WaitGroup{}

// 	maxWorkers := ctxt.GetInner(ctx).Concurrency
// 	workerPool := make(chan struct{}, maxWorkers)

// 	for _, t := range pt.inner {
// 		wg.Add(1)
// 		workerPool <- struct{}{}

// 		// the checkpoint part of context can't be shared between goroutines
// 		// since it's used to trace the stack, so we must create a new layer
// 		// of checkpoint context every time put it into a new goroutine.
// 		go func(ctx context.Context, t Task) {
// 			defer func() {
// 				<-workerPool
// 				wg.Done()
// 			}()
// 			if !isDisplayTask(t) {
// 				if !pt.hideDetailDisplay {
// 					log.Infof("+ [Parallel] - %s", t.String())
// 				}
// 			}
// 			ctxt.GetInner(ctx).Ev.PublishTaskBegin(t)
// 			err := t.Execute(ctx)
// 			ctxt.GetInner(ctx).Ev.PublishTaskFinish(t, err)
// 			if err != nil {
// 				mu.Lock()
// 				if firstError == nil {
// 					firstError = err
// 				}
// 				mu.Unlock()
// 			}
// 		}(checkpoint.NewContext(ctx), t)
// 	}
// 	wg.Wait()
// 	if pt.ignoreError {
// 		return nil
// 	}
// 	return firstError
// }

// // Rollback implements the Task interface
// func (pt *Parallel) Rollback(ctx context.Context) error {
// 	var firstError error
// 	var mu sync.Mutex
// 	wg := sync.WaitGroup{}
// 	for _, t := range pt.inner {
// 		wg.Add(1)

// 		// the checkpoint part of context can't be shared between goroutines
// 		// since it's used to trace the stack, so we must create a new layer
// 		// of checkpoint context every time put it into a new goroutine.
// 		go func(ctx context.Context, t Task) {
// 			defer wg.Done()
// 			err := t.Rollback(ctx)
// 			if err != nil {
// 				mu.Lock()
// 				if firstError == nil {
// 					firstError = err
// 				}
// 				mu.Unlock()
// 			}
// 		}(checkpoint.NewContext(ctx), t)
// 	}
// 	wg.Wait()
// 	return firstError
// }
// }


