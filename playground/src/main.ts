class Person {
    constructor(public name: string) {}

    sayHi() {
        console.log(`Hi, my name is ${this.name}.`);
    }
}

const me = new Person("xfy");

console.log(`Hello ${me.name}!`);
me.sayHi();
