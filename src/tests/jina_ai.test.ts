import { ScrapeURL } from "../lib/jina_ai";
import fetch from "node-fetch";

jest.mock("node-fetch");

const { Response } = jest.requireActual("node-fetch");

describe("ScrapeURL", () => {
  const url = "https://example.com";
  const jinaUrl = `https://r.jina.ai/${url}`;

  beforeEach(() => {
    localStorage.setItem(
      "settings",
      JSON.stringify({ jinaAiApiKey: "test-token" })
    );
  });

  afterEach(() => {
    jest.clearAllMocks();
    localStorage.removeItem("settings");
  });

  it("should fetch data successfully", async () => {
    (fetch as jest.MockedFunction<typeof fetch>).mockResolvedValueOnce(
      new Response("Fetched data")
    );

    const data = await ScrapeURL(url);
    expect(data).toBe("Fetched data");
    expect(fetch).toHaveBeenCalledWith(jinaUrl, expect.any(Object));
  });

  it("should throw an error on fetch failure", async () => {
    (fetch as jest.MockedFunction<typeof fetch>).mockResolvedValueOnce(
      new Response(null, { status: 404 })
    );

    await expect(ScrapeURL(url)).rejects.toThrow("HTTP error! status: 404");
  });

  it("should handle errors during fetch", async () => {
    (fetch as jest.MockedFunction<typeof fetch>).mockRejectedValueOnce(
      new Error("Network error")
    );

    await expect(ScrapeURL(url)).rejects.toThrow("Network error");
  });
});
