
 ![Screenshot 2024-12-06 184420](https://github.com/user-attachments/assets/c05573bb-2f37-40f3-9389-966ff5941f23)



this is https://github.com/x1a7x/SIMPLE-RUST-TEXBOARD (which uses sled db) converted to use postgres. 

Seems to work very well, not tested fully yet tho. Rust IS the right lang, but apps still need to be tested and modified. Lightning fast even on tor because so far its only rust postgres and text. The sled db version is lightning fast, but under heavy load postgres would be best. Granted, not many websites need 2000 posts at the same time, and a lot of the modern solutions like sled db can handle 500 or more posts at once quite well and as fast as it gets. This app aims to be versatile- low amount of users to a high amount of users and still fast overall without being stopped. (given the same hardware as other boards). This Rust app wants to be ultra secure, ultra reliable, and unstoppable. Rust is currently the best choice for that. 


Note--- the looks do NOT matter. The css can make this look like any other board. What this is meant to showcase is the speed and reliability it brings.
