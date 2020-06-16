
type OrderId = i64;

enum Message {
	Ack(OrderId),
	Rej(OrderId),
	PartialFill(OrderId, Qty, Price),
	Filled(OrderId),
}