use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use syn::Attribute;


pub trait Named<N> {
    fn name(&self) -> &N;
}

pub trait Attributed<A: Borrow<Attribute>> {
    fn attribute(&self) -> &A;
}

pub struct SimpleNamed<T, N> {
    inner: T,
    name: N
}

impl<T, N> Deref for SimpleNamed<T, N> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<T, N> DerefMut for SimpleNamed<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<T, N> SimpleNamed<T, N> {
    pub fn create(inner: T, name: N) -> Self {
        Self { inner, name }
    }
}

impl<T, N> Named<N> for SimpleNamed<T, N> {
    fn name(&self) -> &N { &self.name }
}

impl<T, N> Borrow<T> for SimpleNamed<T, N> {
    fn borrow(&self) -> &T {
        &self.inner
    }
}


impl<T, N> From<(T, N)> for SimpleNamed<T, N> {
    fn from((inner, name): (T, N)) -> Self { Self::create(inner, name) }
}

pub struct SimpleAttributed<T, A: Borrow<Attribute>> {
    inner: T,
    attribute: A
}

impl<T, A: Borrow<Attribute>> SimpleAttributed<T, A> {
    pub fn create(inner: T, attribute: A) -> Self {
        Self { inner, attribute }
    }
}

impl<T, A: Borrow<Attribute>> Attributed<A> for SimpleAttributed<T, A> {
    fn attribute(&self) -> &A { &self.attribute }
}

impl<T, A: Borrow<Attribute>> From<(T, A)> for SimpleAttributed<T, A> {
    fn from((inner, attribute): (T, A)) -> Self {
        Self::create(inner, attribute)
    }
}

impl<T, A: Borrow<Attribute>> Borrow<T> for SimpleAttributed<T, A> {
    fn borrow(&self) -> &T {
        &self.inner
    }
}

impl<T, A: Borrow<Attribute>> Deref for SimpleAttributed<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<T, A: Borrow<Attribute>> DerefMut for SimpleAttributed<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

pub struct SimpleNamedAttributed<T, A: Borrow<Attribute>, N> {
    inner: T,
    attribute: A,
    name: N
}

impl<T, A: Borrow<Attribute>, N> SimpleNamedAttributed<T, A, N> {
    pub fn create(inner: T, attribute: A, name: N) -> Self {
        Self { inner, attribute, name }
    }

    pub fn from_named(named: SimpleNamed<T, N>, attribute: A) -> Self {
        let (inner, name) = (named.inner, named.name);
        Self::create(inner, attribute, name)
    }

    pub fn from_attributed(attributed: SimpleAttributed<T, A>, name: N) -> Self {
        let (inner, attribute) = (attributed.inner, attributed.attribute);
        Self::create(inner, attribute, name)
    }

}

impl<T, A: Borrow<Attribute>, N> From<SimpleNamed<SimpleAttributed<T, A>, N>> for SimpleNamedAttributed<T, A, N> {
    fn from(value: SimpleNamed<SimpleAttributed<T, A>, N>) -> Self {
        let (attributed, name) = (value.inner, value.name);
        let (inner, attribute) = (attributed.inner, attributed.attribute);
        Self::create(inner, attribute, name)
    }
}

impl<T, A: Borrow<Attribute>, N> From<SimpleAttributed<SimpleNamed<T, N>, A>> for SimpleNamedAttributed<T, A, N> {
    fn from(value: SimpleAttributed<SimpleNamed<T, N>, A>) -> Self {
        let (named, attribute) = (value.inner, value.attribute);
        let (inner, name) = (named.inner, named.name);
        Self::create(inner, attribute, name)
    }
}

impl<T, A: Borrow<Attribute>, N> Named<N> for SimpleNamedAttributed<T, A, N> {
    fn name(&self) -> &N { &self.name }
}

impl<T, A: Borrow<Attribute>, N> Attributed<A> for SimpleNamedAttributed<T, A, N> {
    fn attribute(&self) -> &A { &self.attribute }
}

impl<T, A: Borrow<Attribute>, N> Deref for SimpleNamedAttributed<T, A, N> {
    type Target = T;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<T, A: Borrow<Attribute>, N> DerefMut for SimpleNamedAttributed<T, A, N> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<T, A: Borrow<Attribute>, N> Borrow<T> for SimpleNamedAttributed<T, A, N> {
    fn borrow(&self) -> &T {
        &self.inner
    }
}