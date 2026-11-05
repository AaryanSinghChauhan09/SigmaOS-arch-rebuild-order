use sigmaos_arch_rebuild_order::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_arch_rebuild_order::new("sigmaos-arch-rebuild-order").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-arch-rebuild-order"));
}
