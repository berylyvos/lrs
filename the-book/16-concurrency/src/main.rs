mod threads;
mod channel;
mod shared_state;
mod send_sync;

fn main() {
    // threads::test_thread_spawn();
    // threads::test_thread_move_ownership();

    channel::test_channel();
    channel::test_multiple_producers();
    shared_state::share_mutex_between_threads_using_arc();
}
