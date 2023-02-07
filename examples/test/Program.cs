using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace test {
    class Account {
        public decimal quantity { get; set; }
    }

    class Person {
        public string name { get; set; }
        public int age { get; set; }
        public Account account { get; set; }
    }

    class Program {
        static void Main(string[] args) {
            var person = new Person();
            person.name = "Danilo";
            person.age = 33;
            person.account = new Account();
            person.account.quantity = 12314;
            string json = JsonSerializer.Serialize(person);
            Console.WriteLine(json);
            Person fromJson = JsonSerializer.Deserialize<Person>(json);
            Console.WriteLine(fromJson.account.quantity);
        }
    }
}
