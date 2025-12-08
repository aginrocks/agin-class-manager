import type { paths } from "@/types/api";
import createFetchClient from "openapi-fetch";
import createClient from "openapi-react-query";

const fetchClient = createFetchClient<paths>({
  baseUrl: "/",
});
export const $api = createClient(fetchClient);

export type createOrgBody =
  paths["/api/organizations"]["post"]["requestBody"]["content"]["application/json"];

export type addMemberBody =
  paths["/api/organizations/{org_id}/members"]["post"]["requestBody"]["content"]["application/json"];
