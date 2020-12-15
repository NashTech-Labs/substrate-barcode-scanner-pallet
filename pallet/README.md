# substrate-barcode-scanner-pallet
This substrate based pallet scans 2D barcode of the product which is tied to a Blockchain system. 

This pallet is based on a [Substrate-Pallet-Template](https://github.com/substrate-developer-hub/substrate-pallet-template).

## What is Barcode Scanner?

Every popular brand has fake manufacturers selling a counterfeited item at cheaper rates. Even the company experts may not be able to distinguish between fake ones and real ones. What if the original manufacturer has embedded a 2D barcode on the product which is tied to a blockchain system.
You can scan the 2D barcode using your smartphone, and your smartphone will tell you whether the product is fake or not.

For now, this pallet covers only two functionalities:

1) Add a product in chain and associate with a barcode
2) Verify a barcode