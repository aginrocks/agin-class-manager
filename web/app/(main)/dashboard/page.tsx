"use client";
import { DataTable } from "@/components/data-table";
import { SectionCards } from "@/components/section-cards";
import { SiteHeader } from "@/components/site-header";

import { useAtomValue } from "jotai";
import { SelectedOrgAtom } from "@/lib/atoms/org";
import { useQuery } from "@tanstack/react-query";
import { $api } from "@/lib/providers/api";
import { useMemo } from "react";
import { UserAtom } from "@/lib/atoms/user";

export default function Page() {
  const org = useAtomValue(SelectedOrgAtom);
  const user = useAtomValue(UserAtom);

  const { data: fundrisings } = useQuery(
    $api.queryOptions(
      "get",
      "/api/organizations/{org_id}/fundraising",
      {
        params: {
          //@ts-expect-error when org._id is null then it's not even enabled and it shouldn't be a problem
          path: { org_id: org?._id },
        },
      },
      {
        enabled: !!org?._id,
      }
    )
  );

  const { data: organization } = useQuery(
    $api.queryOptions(
      "get",
      "/api/organizations/{org_id}",
      {
        params: {
          //@ts-expect-error when org._id is null then it's not even enabled and it shouldn't be a problem
          path: { org_id: org?._id },
        },
      },
      {
        enabled: !!org?._id,
      }
    )
  );

  const totalDue = useMemo(() => {
    if (!fundrisings) return 0;
    return (
      fundrisings.reduce(
        (acc, curr) =>
          acc +
          curr.total_amount -
          (curr.payers.find((u) => u.user_id == user?._id)?.paid_amount || 0),
        0
      ) / (organization?.members.length || 1)
    );
  }, [fundrisings, organization?.members]);

  return (
    <>
      <SiteHeader title="Dashboard" />
      <div className="flex flex-1 flex-col">
        <div className="@container/main flex flex-1 flex-col gap-2">
          <div className="flex flex-col gap-4 px-4 lg:px-6 py-4 md:gap-6 md:py-6">
            <SectionCards
              totalBudget={organization?.budget}
              totalDue={totalDue}
            />
            {/* <div className="px-4 lg:px-6">
              <ChartAreaInteractive />
            </div> */}

            <DataTable data={fundrisings || []} />
          </div>
        </div>
      </div>
    </>
  );
}
