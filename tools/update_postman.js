const Converter = require("openapi-to-postmanv2");

const POSTMAN_COLLECTION_ID =
  process.env.POSTMAN_COLLECTION_ID ||
  "15964941-fdc9fd1f-d7bc-4f7e-b5ce-9189dbcbb81d";
const POSTMAN_API_KEY = process.env.POSTMAN_API_KEY;

const openapiUrl = "https://api.svix.com/api/v1/openapi.json";

const conversionOptions = {
  folderStrategy: "tags",
};

const error = function (msg) {
  console.error(msg);
  process.exit(1);
};

const update_collection = async (collection) => {
  const res = await fetch(
    `https://api.getpostman.com/collections/${POSTMAN_COLLECTION_ID}`,
    {
      method: "PUT",
      headers: {
        "x-api-key": POSTMAN_API_KEY,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ collection }),
    },
  );
  if (!res.ok) {
    error(`Failed to update collection: ${res.status} ${res.statusText}`);
  }
};

(async () => {
  try {
    const res = await fetch(openapiUrl);
    const openapiData = await res.json();

    let collection;
    Converter.convert(
      { type: "json", data: openapiData },
      conversionOptions,
      (_err, conversionResult) => {
        if (!conversionResult.result) {
          error(`Could not convert: ${conversionResult.reason}`);
        }
        collection = conversionResult.output[0].data;
      },
    );

    await update_collection(collection);
  } catch (e) {
    error(e.message ?? e);
  }
})();
