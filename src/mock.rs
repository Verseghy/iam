use lettre::{address::Envelope, transport::smtp::Error as SmtpError, AsyncTransport, Message};
use redis::{aio::ConnectionLike, Cmd, Pipeline, RedisFuture, Value};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

struct MockRedisInner {
    cmds: Vec<Cmd>,
    results: VecDeque<Value>,
}

#[derive(Clone)]
pub struct MockRedis {
    inner: Arc<Mutex<MockRedisInner>>,
}

impl MockRedis {
    pub fn new(results: Vec<Value>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(MockRedisInner {
                cmds: Vec::new(),
                results: VecDeque::from(results),
            })),
        }
    }

    pub fn cmds(&self) -> Vec<Cmd> {
        let inner = self.inner.lock().unwrap();
        inner.cmds.clone()
    }
}

impl ConnectionLike for MockRedis {
    fn req_packed_command<'a>(&'a mut self, cmd: &'a Cmd) -> RedisFuture<'a, Value> {
        Box::pin(async {
            let mut inner = self.inner.lock().unwrap();
            inner.cmds.push(cmd.clone());
            Ok(inner.results.pop_front().unwrap())
        })
    }

    fn req_packed_commands<'a>(
        &'a mut self,
        cmd: &'a Pipeline,
        _offset: usize,
        _count: usize,
    ) -> RedisFuture<'a, Vec<Value>> {
        Box::pin(async move {
            let mut inner = self.inner.lock().unwrap();
            let mut res = Vec::new();
            for cmd in cmd.cmd_iter() {
                inner.cmds.push(cmd.clone());
                res.push(inner.results.pop_front().unwrap());
            }
            Ok(res)
        })
    }

    fn get_db(&self) -> i64 {
        0
    }
}

struct MockSmtpTransportInner {
    success: bool,
    message: Option<Message>,
}

#[derive(Clone)]
pub struct MockSmtpTransport {
    inner: Arc<Mutex<MockSmtpTransportInner>>,
}

impl MockSmtpTransport {
    pub fn new(success: bool) -> Self {
        Self {
            inner: Arc::new(Mutex::new(MockSmtpTransportInner {
                success,
                message: None,
            })),
        }
    }

    pub fn message(&self) -> Message {
        let inner = self.inner.lock().unwrap();
        inner.message.as_ref().unwrap().clone()
    }
}

#[async_trait::async_trait]
impl AsyncTransport for MockSmtpTransport {
    type Ok = ();
    type Error = SmtpError;

    async fn send(&self, message: Message) -> Result<Self::Ok, Self::Error> {
        let mut inner = self.inner.lock().unwrap();
        inner.message = Some(message);

        Ok(())
    }

    async fn send_raw(&self, _envelope: &Envelope, _email: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub fn assert_cmds(cmds: &[Cmd], expected: &[Vec<u8>]) {
    assert_eq!(cmds.len(), expected.len());

    for (a, b) in cmds.iter().zip(expected) {
        assert_eq!(a.get_packed_command(), *b);
    }
}

pub fn redis_cmd(args: &[&str]) -> Vec<u8> {
    redis::cmd(args[0]).arg(&args[1..]).get_packed_command()
}
