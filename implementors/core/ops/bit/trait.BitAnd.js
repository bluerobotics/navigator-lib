(function() {var implementors = {
"bitvec":[["impl&lt;T, O, Rhs&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;Rhs&gt; for <a class=\"struct\" href=\"bitvec/boxed/struct.BitBox.html\" title=\"struct bitvec::boxed::BitBox\">BitBox</a>&lt;T, O&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    <a class=\"struct\" href=\"bitvec/slice/struct.BitSlice.html\" title=\"struct bitvec::slice::BitSlice\">BitSlice</a>&lt;T, O&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html\" title=\"trait core::ops::bit::BitAndAssign\">BitAndAssign</a>&lt;Rhs&gt;,</span>"],["impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;R&gt; for <a class=\"struct\" href=\"bitvec/index/struct.BitMask.html\" title=\"struct bitvec::index::BitMask\">BitMask</a>&lt;R&gt;<span class=\"where fmt-newline\">where\n    R: <a class=\"trait\" href=\"bitvec/mem/trait.BitRegister.html\" title=\"trait bitvec::mem::BitRegister\">BitRegister</a>,</span>"],["impl&lt;T, O, Rhs&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;Rhs&gt; for <a class=\"struct\" href=\"bitvec/vec/struct.BitVec.html\" title=\"struct bitvec::vec::BitVec\">BitVec</a>&lt;T, O&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    <a class=\"struct\" href=\"bitvec/slice/struct.BitSlice.html\" title=\"struct bitvec::slice::BitSlice\">BitSlice</a>&lt;T, O&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html\" title=\"trait core::ops::bit::BitAndAssign\">BitAndAssign</a>&lt;Rhs&gt;,</span>"],["impl&lt;A, O, Rhs&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;Rhs&gt; for <a class=\"struct\" href=\"bitvec/array/struct.BitArray.html\" title=\"struct bitvec::array::BitArray\">BitArray</a>&lt;A, O&gt;<span class=\"where fmt-newline\">where\n    A: <a class=\"trait\" href=\"bitvec/view/trait.BitViewSized.html\" title=\"trait bitvec::view::BitViewSized\">BitViewSized</a>,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    <a class=\"struct\" href=\"bitvec/slice/struct.BitSlice.html\" title=\"struct bitvec::slice::BitSlice\">BitSlice</a>&lt;A::<a class=\"associatedtype\" href=\"bitvec/view/trait.BitView.html#associatedtype.Store\" title=\"type bitvec::view::BitView::Store\">Store</a>, O&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html\" title=\"trait core::ops::bit::BitAndAssign\">BitAndAssign</a>&lt;Rhs&gt;,</span>"]],
"gpio_cdev":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"gpio_cdev/struct.EventRequestFlags.html\" title=\"struct gpio_cdev::EventRequestFlags\">EventRequestFlags</a>&gt; for <a class=\"struct\" href=\"gpio_cdev/struct.EventRequestFlags.html\" title=\"struct gpio_cdev::EventRequestFlags\">EventRequestFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"gpio_cdev/struct.LineRequestFlags.html\" title=\"struct gpio_cdev::LineRequestFlags\">LineRequestFlags</a>&gt; for <a class=\"struct\" href=\"gpio_cdev/struct.LineRequestFlags.html\" title=\"struct gpio_cdev::LineRequestFlags\">LineRequestFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"gpio_cdev/struct.LineFlags.html\" title=\"struct gpio_cdev::LineFlags\">LineFlags</a>&gt; for <a class=\"struct\" href=\"gpio_cdev/struct.LineFlags.html\" title=\"struct gpio_cdev::LineFlags\">LineFlags</a>"]],
"i2cdev":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"i2cdev/linux/struct.I2CMessageFlags.html\" title=\"struct i2cdev::linux::I2CMessageFlags\">I2CMessageFlags</a>&gt; for <a class=\"struct\" href=\"i2cdev/linux/struct.I2CMessageFlags.html\" title=\"struct i2cdev::linux::I2CMessageFlags\">I2CMessageFlags</a>"]],
"nix":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/timerfd/struct.TimerSetTimeFlags.html\" title=\"struct nix::sys::timerfd::TimerSetTimeFlags\">TimerSetTimeFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/timerfd/struct.TimerSetTimeFlags.html\" title=\"struct nix::sys::timerfd::TimerSetTimeFlags\">TimerSetTimeFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/termios/struct.ControlFlags.html\" title=\"struct nix::sys::termios::ControlFlags\">ControlFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/termios/struct.ControlFlags.html\" title=\"struct nix::sys::termios::ControlFlags\">ControlFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/eventfd/struct.EfdFlags.html\" title=\"struct nix::sys::eventfd::EfdFlags\">EfdFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/eventfd/struct.EfdFlags.html\" title=\"struct nix::sys::eventfd::EfdFlags\">EfdFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/fcntl/struct.OFlag.html\" title=\"struct nix::fcntl::OFlag\">OFlag</a>&gt; for <a class=\"struct\" href=\"nix/fcntl/struct.OFlag.html\" title=\"struct nix::fcntl::OFlag\">OFlag</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/inotify/struct.InitFlags.html\" title=\"struct nix::sys::inotify::InitFlags\">InitFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/inotify/struct.InitFlags.html\" title=\"struct nix::sys::inotify::InitFlags\">InitFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/socket/struct.MsgFlags.html\" title=\"struct nix::sys::socket::MsgFlags\">MsgFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/socket/struct.MsgFlags.html\" title=\"struct nix::sys::socket::MsgFlags\">MsgFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/fcntl/struct.RenameFlags.html\" title=\"struct nix::fcntl::RenameFlags\">RenameFlags</a>&gt; for <a class=\"struct\" href=\"nix/fcntl/struct.RenameFlags.html\" title=\"struct nix::fcntl::RenameFlags\">RenameFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/inotify/struct.AddWatchFlags.html\" title=\"struct nix::sys::inotify::AddWatchFlags\">AddWatchFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/inotify/struct.AddWatchFlags.html\" title=\"struct nix::sys::inotify::AddWatchFlags\">AddWatchFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/poll/struct.PollFlags.html\" title=\"struct nix::poll::PollFlags\">PollFlags</a>&gt; for <a class=\"struct\" href=\"nix/poll/struct.PollFlags.html\" title=\"struct nix::poll::PollFlags\">PollFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/stat/struct.SFlag.html\" title=\"struct nix::sys::stat::SFlag\">SFlag</a>&gt; for <a class=\"struct\" href=\"nix/sys/stat/struct.SFlag.html\" title=\"struct nix::sys::stat::SFlag\">SFlag</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/mount/struct.MsFlags.html\" title=\"struct nix::mount::MsFlags\">MsFlags</a>&gt; for <a class=\"struct\" href=\"nix/mount/struct.MsFlags.html\" title=\"struct nix::mount::MsFlags\">MsFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/fcntl/struct.FallocateFlags.html\" title=\"struct nix::fcntl::FallocateFlags\">FallocateFlags</a>&gt; for <a class=\"struct\" href=\"nix/fcntl/struct.FallocateFlags.html\" title=\"struct nix::fcntl::FallocateFlags\">FallocateFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/signalfd/struct.SfdFlags.html\" title=\"struct nix::sys::signalfd::SfdFlags\">SfdFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/signalfd/struct.SfdFlags.html\" title=\"struct nix::sys::signalfd::SfdFlags\">SfdFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/kmod/struct.ModuleInitFlags.html\" title=\"struct nix::kmod::ModuleInitFlags\">ModuleInitFlags</a>&gt; for <a class=\"struct\" href=\"nix/kmod/struct.ModuleInitFlags.html\" title=\"struct nix::kmod::ModuleInitFlags\">ModuleInitFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/fcntl/struct.SealFlag.html\" title=\"struct nix::fcntl::SealFlag\">SealFlag</a>&gt; for <a class=\"struct\" href=\"nix/fcntl/struct.SealFlag.html\" title=\"struct nix::fcntl::SealFlag\">SealFlag</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/termios/struct.OutputFlags.html\" title=\"struct nix::sys::termios::OutputFlags\">OutputFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/termios/struct.OutputFlags.html\" title=\"struct nix::sys::termios::OutputFlags\">OutputFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/fcntl/struct.SpliceFFlags.html\" title=\"struct nix::fcntl::SpliceFFlags\">SpliceFFlags</a>&gt; for <a class=\"struct\" href=\"nix/fcntl/struct.SpliceFFlags.html\" title=\"struct nix::fcntl::SpliceFFlags\">SpliceFFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/unistd/struct.AccessFlags.html\" title=\"struct nix::unistd::AccessFlags\">AccessFlags</a>&gt; for <a class=\"struct\" href=\"nix/unistd/struct.AccessFlags.html\" title=\"struct nix::unistd::AccessFlags\">AccessFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/mman/struct.MsFlags.html\" title=\"struct nix::sys::mman::MsFlags\">MsFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/mman/struct.MsFlags.html\" title=\"struct nix::sys::mman::MsFlags\">MsFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/quota/struct.QuotaValidFlags.html\" title=\"struct nix::sys::quota::QuotaValidFlags\">QuotaValidFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/quota/struct.QuotaValidFlags.html\" title=\"struct nix::sys::quota::QuotaValidFlags\">QuotaValidFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/mqueue/struct.FdFlag.html\" title=\"struct nix::mqueue::FdFlag\">FdFlag</a>&gt; for <a class=\"struct\" href=\"nix/mqueue/struct.FdFlag.html\" title=\"struct nix::mqueue::FdFlag\">FdFlag</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/epoll/struct.EpollFlags.html\" title=\"struct nix::sys::epoll::EpollFlags\">EpollFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/epoll/struct.EpollFlags.html\" title=\"struct nix::sys::epoll::EpollFlags\">EpollFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/stat/struct.Mode.html\" title=\"struct nix::sys::stat::Mode\">Mode</a>&gt; for <a class=\"struct\" href=\"nix/sys/stat/struct.Mode.html\" title=\"struct nix::sys::stat::Mode\">Mode</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/ptrace/struct.Options.html\" title=\"struct nix::sys::ptrace::Options\">Options</a>&gt; for <a class=\"struct\" href=\"nix/sys/ptrace/struct.Options.html\" title=\"struct nix::sys::ptrace::Options\">Options</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/fcntl/struct.AtFlags.html\" title=\"struct nix::fcntl::AtFlags\">AtFlags</a>&gt; for <a class=\"struct\" href=\"nix/fcntl/struct.AtFlags.html\" title=\"struct nix::fcntl::AtFlags\">AtFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/wait/struct.WaitPidFlag.html\" title=\"struct nix::sys::wait::WaitPidFlag\">WaitPidFlag</a>&gt; for <a class=\"struct\" href=\"nix/sys/wait/struct.WaitPidFlag.html\" title=\"struct nix::sys::wait::WaitPidFlag\">WaitPidFlag</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/mman/struct.MlockAllFlags.html\" title=\"struct nix::sys::mman::MlockAllFlags\">MlockAllFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/mman/struct.MlockAllFlags.html\" title=\"struct nix::sys::mman::MlockAllFlags\">MlockAllFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/signal/struct.SaFlags.html\" title=\"struct nix::sys::signal::SaFlags\">SaFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/signal/struct.SaFlags.html\" title=\"struct nix::sys::signal::SaFlags\">SaFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/mman/struct.MapFlags.html\" title=\"struct nix::sys::mman::MapFlags\">MapFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/mman/struct.MapFlags.html\" title=\"struct nix::sys::mman::MapFlags\">MapFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/fcntl/struct.FdFlag.html\" title=\"struct nix::fcntl::FdFlag\">FdFlag</a>&gt; for <a class=\"struct\" href=\"nix/fcntl/struct.FdFlag.html\" title=\"struct nix::fcntl::FdFlag\">FdFlag</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/kmod/struct.DeleteModuleFlags.html\" title=\"struct nix::kmod::DeleteModuleFlags\">DeleteModuleFlags</a>&gt; for <a class=\"struct\" href=\"nix/kmod/struct.DeleteModuleFlags.html\" title=\"struct nix::kmod::DeleteModuleFlags\">DeleteModuleFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/personality/struct.Persona.html\" title=\"struct nix::sys::personality::Persona\">Persona</a>&gt; for <a class=\"struct\" href=\"nix/sys/personality/struct.Persona.html\" title=\"struct nix::sys::personality::Persona\">Persona</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/termios/struct.InputFlags.html\" title=\"struct nix::sys::termios::InputFlags\">InputFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/termios/struct.InputFlags.html\" title=\"struct nix::sys::termios::InputFlags\">InputFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/termios/struct.LocalFlags.html\" title=\"struct nix::sys::termios::LocalFlags\">LocalFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/termios/struct.LocalFlags.html\" title=\"struct nix::sys::termios::LocalFlags\">LocalFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/timerfd/struct.TimerFlags.html\" title=\"struct nix::sys::timerfd::TimerFlags\">TimerFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/timerfd/struct.TimerFlags.html\" title=\"struct nix::sys::timerfd::TimerFlags\">TimerFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/net/if_/struct.InterfaceFlags.html\" title=\"struct nix::net::if_::InterfaceFlags\">InterfaceFlags</a>&gt; for <a class=\"struct\" href=\"nix/net/if_/struct.InterfaceFlags.html\" title=\"struct nix::net::if_::InterfaceFlags\">InterfaceFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sched/struct.CloneFlags.html\" title=\"struct nix::sched::CloneFlags\">CloneFlags</a>&gt; for <a class=\"struct\" href=\"nix/sched/struct.CloneFlags.html\" title=\"struct nix::sched::CloneFlags\">CloneFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/mqueue/struct.MQ_OFlag.html\" title=\"struct nix::mqueue::MQ_OFlag\">MQ_OFlag</a>&gt; for <a class=\"struct\" href=\"nix/mqueue/struct.MQ_OFlag.html\" title=\"struct nix::mqueue::MQ_OFlag\">MQ_OFlag</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/mount/struct.MntFlags.html\" title=\"struct nix::mount::MntFlags\">MntFlags</a>&gt; for <a class=\"struct\" href=\"nix/mount/struct.MntFlags.html\" title=\"struct nix::mount::MntFlags\">MntFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/socket/struct.SockFlag.html\" title=\"struct nix::sys::socket::SockFlag\">SockFlag</a>&gt; for <a class=\"struct\" href=\"nix/sys/socket/struct.SockFlag.html\" title=\"struct nix::sys::socket::SockFlag\">SockFlag</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/mman/struct.ProtFlags.html\" title=\"struct nix::sys::mman::ProtFlags\">ProtFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/mman/struct.ProtFlags.html\" title=\"struct nix::sys::mman::ProtFlags\">ProtFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/epoll/struct.EpollCreateFlags.html\" title=\"struct nix::sys::epoll::EpollCreateFlags\">EpollCreateFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/epoll/struct.EpollCreateFlags.html\" title=\"struct nix::sys::epoll::EpollCreateFlags\">EpollCreateFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/mman/struct.MRemapFlags.html\" title=\"struct nix::sys::mman::MRemapFlags\">MRemapFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/mman/struct.MRemapFlags.html\" title=\"struct nix::sys::mman::MRemapFlags\">MRemapFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/memfd/struct.MemFdCreateFlag.html\" title=\"struct nix::sys::memfd::MemFdCreateFlag\">MemFdCreateFlag</a>&gt; for <a class=\"struct\" href=\"nix/sys/memfd/struct.MemFdCreateFlag.html\" title=\"struct nix::sys::memfd::MemFdCreateFlag\">MemFdCreateFlag</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"nix/sys/statvfs/struct.FsFlags.html\" title=\"struct nix::sys::statvfs::FsFlags\">FsFlags</a>&gt; for <a class=\"struct\" href=\"nix/sys/statvfs/struct.FsFlags.html\" title=\"struct nix::sys::statvfs::FsFlags\">FsFlags</a>"]],
"rustix":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/termios/struct.OutputModes.html\" title=\"struct rustix::termios::OutputModes\">OutputModes</a>&gt; for <a class=\"struct\" href=\"rustix/termios/struct.OutputModes.html\" title=\"struct rustix::termios::OutputModes\">OutputModes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/io/struct.FdFlags.html\" title=\"struct rustix::io::FdFlags\">FdFlags</a>&gt; for <a class=\"struct\" href=\"rustix/io/struct.FdFlags.html\" title=\"struct rustix::io::FdFlags\">FdFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/io/struct.DupFlags.html\" title=\"struct rustix::io::DupFlags\">DupFlags</a>&gt; for <a class=\"struct\" href=\"rustix/io/struct.DupFlags.html\" title=\"struct rustix::io::DupFlags\">DupFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/termios/struct.InputModes.html\" title=\"struct rustix::termios::InputModes\">InputModes</a>&gt; for <a class=\"struct\" href=\"rustix/termios/struct.InputModes.html\" title=\"struct rustix::termios::InputModes\">InputModes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/termios/struct.LocalModes.html\" title=\"struct rustix::termios::LocalModes\">LocalModes</a>&gt; for <a class=\"struct\" href=\"rustix/termios/struct.LocalModes.html\" title=\"struct rustix::termios::LocalModes\">LocalModes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/termios/struct.ControlModes.html\" title=\"struct rustix::termios::ControlModes\">ControlModes</a>&gt; for <a class=\"struct\" href=\"rustix/termios/struct.ControlModes.html\" title=\"struct rustix::termios::ControlModes\">ControlModes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/io/struct.ReadWriteFlags.html\" title=\"struct rustix::io::ReadWriteFlags\">ReadWriteFlags</a>&gt; for <a class=\"struct\" href=\"rustix/io/struct.ReadWriteFlags.html\" title=\"struct rustix::io::ReadWriteFlags\">ReadWriteFlags</a>"]],
"spidev":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"spidev/struct.SpiModeFlags.html\" title=\"struct spidev::SpiModeFlags\">SpiModeFlags</a>&gt; for <a class=\"struct\" href=\"spidev/struct.SpiModeFlags.html\" title=\"struct spidev::SpiModeFlags\">SpiModeFlags</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()