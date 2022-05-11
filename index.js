console.log("welcome to node.......")

let mp=new Map()

// //adding values to the map 
mp.set("a",65);
mp.set("b",66);

// logging map object

mp.forEach((values,keys)=>{
    document.write("values: ",values+". keys+ "<br>")
}
let food = ['mango', 'rice', 'pepper', 'pear'];
food.forEach(function(foodItem){ console.log('I want to eat'+foodItem);});

// map

let cost = [100,400,300,700];
let newCost = cost.map(function(costItem){return costItem /10;
});
console.log(newCost);


// filter 
let cost2 = [100,400,50,40,700];
let smallCost = cost2.filter(function(costItem){return costItem < 200
});
console.log(smallCost);

// reduce 

const numbers = [1,2,3,4];
const sum = numbers.reduce(function (result, item ) {
    return result + item;
});

console.log(sum);


// chaining operations


var http = require('http');
http.createServer(function(req, res){
    res.writeHead(200, {'Content-Type': 'text/plain'});
    res.end('Hello world!');
}).listen(8080);