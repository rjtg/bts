# Papers on the office desk

Open **Research** on the desk to see the binder. Its tabs open Survey orders,
Young viewers, Households and Specialists. An audience file keeps its
published snapshot beside the Insight control. Switch to Survey orders to
set shared polling. Leaving a file keeps its unsubmitted dial value.

Open the **Production** clipboard to see the Studio book. A reviewed
discussion concept opens Work orders. Sway requests, Hiring, Newsroom and
Facilities have their own sheets. Film jobs opens the film folders in the
same place. Longer sheets scroll within their cover. Close the cover to
return to the room and its prop.

- [Research binder on the blotter](research-binder.png)
- [Studio work orders on the blotter](studio-work-orders.png)

The screenshot journey in `web/tests/office-papers.spec.ts` opens both props
from the resting room. It checks the selected page, draft retention and the
narrow layout. To capture it from a rebuilt bundle:

```sh
cd web
BTS_OFFICE_PAPERS_CAPTURES=../contrib/office-papers npm run test:e2e -- office-papers
```
