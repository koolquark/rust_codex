use std::{
    future::Future,
    pin::Pin,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

// Future executor is a top level executor.
// -- Flow --
// Executor polls future once to start off
// Future indicated readyness by calling wake()
// Executor places future in queue
// Executor calls poll again
// repeats until future has completed.
// -----
// Executor receives tasks off a channel and runs them
// ready_queue holds the mpsc receiver

// The system is implemented using the following
// 1. Executor
// 2. Spawner
// 3. Task
// 4. MPSC Channel
// 5. Waker
// Executor holds rx side of an mpsc channel through which it receives
// tasks. Each task has a future inside it which is polled by the executor.
// Spawner holds the tx end of an mpsc channel. Its the spawner who puts
// futures onto the channel ( to be received by executor)
// Spawner boxes the future, wraps in a Mutex and places inside the task structure,
// then its send over the mpsc channel
// The task structure holds a refrence to the sender side of the mpsc channel,
// this is required for executor to place task back on to task queue (??)
// The purpose of waker is to tell executor which tasks are ready.
// Thus executor polls only tasks that are ready to make progress
// When Tasks wake_by_ref is called, the task is cloned and send
// back to the channel via task_sender mpsc tx. There is one task per
// future. Its the future which invoked the wake.
//

struct Executor {
    // the mpsc receiver side , receives tasks to be polled
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}
impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender
            .send(task)
            .expect("too many requests in queue");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("to many tasks queued")
    }
}

impl Executor {
    fn run(&self) {
        // take an item from the mpsc channel
        // the item is a task. We have one task per future that needs to be run to completion

        while let Ok(task) = self.ready_queue.recv() {
            // get a lock , take out the future
            let mut future_slot = task.future.lock().unwrap();

            if let Some(mut future) = future_slot.take() {
                // the waker_ref function creates a reference to a Waker from
                // a type that implements ArcWake
                // We need a 'Future' compatible Waker, that will get filled in the context
                // supplied via poll function of the Future.
                let waker_r = waker_ref(&task);
                // Construct the context that will be supplied to the poll function of the
                // Future.
                // Here &*waker_r ie &(*waker_r) is passed to from_waker
                // waker_r ie WakerRef here implements Deref
                // --
                // impl Deref for WakerRef<'_> {
                //   type Target = Waker;

                //   #[inline]
                //   fn deref(&self) -> &Waker {
                //       &self.waker
                //   }
                // }
                // ---
                // So the result of *waker_r is &Waker
                let context = &mut Context::from_waker(&*waker_r);

                if future.as_mut().poll(context).is_pending() {
                    // still pending , put it back in the slot
                    *future_slot = Some(future);
                }
            }
        }
    }
}
struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();

        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}
fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("howdy");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done");
    });

    drop(spawner);

    executor.run();
}
