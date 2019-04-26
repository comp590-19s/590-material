let aRegExp = /^(a+b|c*d)$/;

let input = "abc";
if (aRegExp.test(input)) {
    console.log(`${input} accepted`);
} else {
    console.log(`${input} not accepted`);
}
