import { Worker, NearAccount, parseNEAR } from "near-workspaces";
import anyTest, { TestFn } from "ava";

const test = anyTest as TestFn<{
	worker: Worker;
	accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
	// Init the worker and start a Sandbox server
	const worker = await Worker.init();

	// Deploy contract
	const root = worker.rootAccount;
	const contract = await root.createSubAccount("test-account");
	// Get wasm file path from package.json test script in folder above
	await contract.deploy("./out/main.wasm");

	await root.call(contract.accountId, "new", {
		owner_id: root.accountId,
	});
	// Save state for test runs, it is unique for each test
	t.context.worker = worker;
	t.context.accounts = { root, contract };
});

test.afterEach(async (t) => {
	// Stop Sandbox server
	await t.context.worker.tearDown().catch((error) => {
		console.log("Failed to stop the Sandbox:", error);
	});
});

test("Storage deposit", async (t) => {
	const { root, contract } = t.context.accounts;
	await root.call(
		contract.accountId,
		"storage_deposit",
		{},
		{
			attachedDeposit: parseNEAR("1 NEAR"),
		}
	);
	const message: string = await root.call(
		contract.accountId,
		"storage_deposit",
		{},
		{
			attachedDeposit: parseNEAR("2 NEAR"),
		}
	);
	console.log(message);
});
test("STORAGE DEPOSIT SET AND REMOVE", async (t) => {
	const { root, contract } = t.context.accounts;
	await root.call(
		contract.accountId,
		"storage_deposit",
		{},
		{
			attachedDeposit: parseNEAR("10 NEAR"),
		}
	);

	await root.call(contract.accountId, "set_data", {
		string: "Hello World",
	});
	const balance1: any = await contract.view("storage_balance_of", {
		account_id: root.accountId,
	});

	await root.call(contract.accountId, "set_data", {
		string:
			"I AM A LONGER STRING WHICH TAKES UP MORE SPACE AND THEREFORE COSTS MORE TO STORE, I AM A LONGER STRING WHICH TAKES UP MORE SPACE AND THEREFORE COSTS MORE TO STORE",
	});
	const balance2: any = await contract.view("storage_balance_of", {
		account_id: root.accountId,
	});
	await root.call(contract.accountId, "set_data", {
		string: "Hello World",
	});
	const balance3: any = await contract.view("storage_balance_of", {
		account_id: root.accountId,
	});
	t.is(balance2.available < balance1.available, true);
	t.is(balance3.available > balance2.available, true);
});

test("Throw error without deposit", async (t) => {
	const { root, contract } = t.context.accounts;

	await t.throwsAsync(
		root.call(contract.accountId, "set_data", {
			string: "Hello World",
		})
	);
});
