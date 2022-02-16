use core::{
    pin::Pin,
    task::{Context, Poll},
};

use alloc::{boxed::Box, vec::Vec};
use futures_lite::{Future, FutureExt};
use pin_project_lite::pin_project;

pin_project! {
    pub struct ZipArray<T> {
        arr: Vec<(Pin<Box<dyn Future<Output = T> + Send>>, Option<T>)>,
    }
}

impl<T> Future for ZipArray<T> {
    type Output = Vec<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let mut is_pending = true;

        for fu in &mut this.arr.iter_mut() {
            if fu.1.is_none() {
                if let Poll::Ready(output) = fu.0.poll(cx) {
                    fu.1 = Some(output);
                } else {
                    is_pending = true;
                }
            }
        }

        if is_pending {
            Poll::Pending
        } else {
            let mut res = Vec::with_capacity(this.arr.len());

            for fu in this.arr {
                let opt = core::mem::take(&mut fu.1);

                let output = opt.expect("Logic error!!!!!!");

                res.push(output)
            }

            Poll::Ready(res)
        }
    }
}

pub fn zip_array<T, F>(fs: Vec<F>) -> ZipArray<T>
where
    F: Future<Output = T> + Send + 'static,
{
    let mut arr = Vec::with_capacity(fs.len());

    for f in fs {
        let fu: Pin<Box<dyn Future<Output = T> + Send>> = Box::pin(f);
        arr.push((fu, None));
    }

    ZipArray { arr }
}
