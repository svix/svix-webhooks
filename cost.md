## Here's the estimated monthly cost for Canada Central based on current Azure pricing:

Resource	Spec	Est. Monthly Cost
Container Apps	0.5 vCPU / 1 GiB RAM, 1 replica always-on	~$13–18
PostgreSQL Flexible Server	Standard_B1ms, Burstable, 32 GB storage	~$25–30
Azure Cache for Redis	Basic C0 (250 MB)	~$16
Azure Container Registry	Basic SKU	~$5
Log Analytics	Auto-created for Container Apps env	~$2–5
Networking/egress	Outbound webhook deliveries	~$1–3
Total estimate: ~$60–75/month

## Key cost drivers:

PostgreSQL is the biggest cost — even the smallest Burstable tier runs 24/7
Redis Basic C0 has no SLA and is the minimum tier; upgrading to Standard for Redis 6.2 would add ~$50/month
Container Apps charges per vCPU-second and GB-second when running, but with --min-replicas 1 it runs continuously
To reduce costs:

Set --min-replicas 0 on the Container App (scales to zero when idle, saves ~$13/month but has cold start)
PostgreSQL can be stopped manually when not in use (Azure stops it automatically after 7 days idle on Burstable tier)