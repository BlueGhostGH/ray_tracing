use crate::const_generics::{Assert, True};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Progress<const N: usize> {
    buffer: [u8; N],
    pub(crate) status: usize,
}

impl<const N: usize> Progress<N>
where
    Assert<{ N > 0 }>: True,
    [(); N + 3]: Sized,
{
    pub(crate) fn new() -> Self {
        let buffer = [b' '; N];

        Progress { buffer, status: 0 }
    }

    pub(crate) fn advance(&mut self) {
        if self.status == 0 {
            unsafe {
                *self.buffer.get_unchecked_mut(self.status) = b'>';
            }
            self.status += 1;
        } else if self.status == N {
            unsafe {
                *self.buffer.get_unchecked_mut(self.status - 1) = b'=';
            }
        } else {
            unsafe {
                *self.buffer.get_unchecked_mut(self.status) = b'>';
                *self.buffer.get_unchecked_mut(self.status - 1) = b'=';
                self.status += 1;
            }
        }
    }

    pub(crate) fn bar_cr(&self) -> [u8; N + 3] {
        let bar = unsafe {
            let mut bar = [0; N + 3];

            let [cr, opening, closing] = bar.get_many_unchecked_mut([0, 1, N + 2]);
            *cr = b'\r';
            *opening = b'[';
            *closing = b']';

            (&mut bar[2..N + 2]).clone_from_slice(&self.buffer);

            bar
        };

        bar
    }
}
