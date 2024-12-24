use test_11::add_two;
// This is the integration test file
// This is used to test public facing functions
// so that we can simulate real uses for the API
#[test]
fn it_adds_two_2(){
    let result = add_two(3);
    assert_eq!(result, 5);
}