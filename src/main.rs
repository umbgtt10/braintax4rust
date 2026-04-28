use braintax::Braintax;

fn main() {
    let score = Braintax::new().with_nesting(3).with_branches(5).compute();

    println!("Cognitive tax score: {score}");

    let baseline = Braintax::new().compute();
    println!("Baseline (no complexity): {baseline}");

    let deeply_nested = Braintax::new().with_nesting(6).with_branches(12).compute();
    println!("Deeply nested code:      {deeply_nested}");
}
