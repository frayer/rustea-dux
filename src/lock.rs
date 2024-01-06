use std::sync::{Arc, Condvar, Mutex};

#[derive(Clone)]
pub struct Lock {
    lock: Arc<(Mutex<bool>, Condvar)>,
}

impl Lock {
    pub fn new() -> Self {
        Self {
            lock: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    pub fn lock(&self) {
        let (lock, cvar) = &*self.lock;
        {
            let mut locked = lock.lock().unwrap();
            *locked = true;
        }
        cvar.notify_all();
    }

    pub fn unlock(&self) {
        let (lock, cvar) = &*self.lock;
        {
            let mut locked = lock.lock().unwrap();
            *locked = false;
        }
        cvar.notify_all();
    }

    pub fn wait(&self) {
        let (lock, cvar) = &*self.lock;
        let mut locked = lock.lock().unwrap();
        while *locked {
            locked = cvar.wait(locked).unwrap();
        }
    }
}
