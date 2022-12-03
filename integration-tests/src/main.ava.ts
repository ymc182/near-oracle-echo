import { Worker, NearAccount, parseNEAR, createKeyPair } from "near-workspaces";
import anyTest, { TestFn } from "ava";

const test = anyTest as TestFn<{
	worker: Worker;
	accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
	// Init the worker and start a Sandbox server
	const worker = await Worker.init({
		network: "testnet",
		testnetMasterAccountId: "ewtd.testnet",
		initialBalance: "100000000000000000000000000",
	});

	// Deploy contract
	const root = worker.rootAccount;

	const contract = await root.devDeploy("../out/main.wasm", {
		initialBalance: parseNEAR("9 NEAR"),
	});

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

	let oracle_res: any = await root.call(
		contract.accountId,
		"create_oracle",
		{
			url: "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd",
			data: JSON.stringify({ ethereum: { usd: 2000 } }),
		},
		{
			attachedDeposit: parseNEAR("0.000001 NEAR"),
		}
	);
	console.log(JSON.parse(oracle_res.data));
});
