import type { TokenData } from "$lib/types";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ parent }) => {
  const parentData: TokenData = (await parent()) as TokenData;
  return {
    token: parentData.token,
  };
};
