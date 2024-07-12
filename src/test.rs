use crate::ApplyIf;

#[derive(Default)]
struct ImmutableBuilder {
    one: i32,
    two: i32,
}

impl ImmutableBuilder {
    pub fn first(self, val: i32) -> Self {
        Self { one: val, ..self }
    }
    pub fn second(self, val: i32) -> Self {
        Self { two: val, ..self }
    }
    pub fn build(self) -> (i32, i32) {
        (self.one, self.two)
    }
}

#[derive(Default)]
struct MutableBuilder {
    one: i32,
    two: i32,
}

impl MutableBuilder {
    pub fn first(&mut self, val: i32) -> &mut Self {
        self.one = val;
        self
    }
    pub fn second(&mut self, val: i32) -> &mut Self {
        self.two = val;
        self
    }
    pub fn build(&mut self) -> (i32, i32) {
        (self.one, self.two)
    }
}

#[test]
fn test_mutable_builder() {
    assert_eq!(MutableBuilder::default().build(), (0, 0));
    assert_eq!(MutableBuilder::default().first(1).second(2).build(), (1, 2));
    assert_eq!(
        MutableBuilder::default()
            .apply_if_mut(true, |b| b.first(1))
            .second(2)
            .build(),
        (1, 2)
    );
    assert_eq!(
        MutableBuilder::default()
            .apply_if_mut(false, |b| b.first(1))
            .second(2)
            .build(),
        (0, 2)
    );
}

#[test]
fn test_immutable_builder() {
    assert_eq!(ImmutableBuilder::default().build(), (0, 0));
    assert_eq!(
        ImmutableBuilder::default().first(1).second(2).build(),
        (1, 2)
    );
    assert_eq!(
        ImmutableBuilder::default()
            .apply_if(true, |b| b.first(1))
            .second(2)
            .build(),
        (1, 2)
    );
    assert_eq!(
        ImmutableBuilder::default()
            .apply_if(false, |b| b.first(1))
            .second(2)
            .build(),
        (0, 2)
    );
}
