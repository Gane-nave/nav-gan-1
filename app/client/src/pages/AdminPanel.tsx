/**
 * G.A.N.E — Master Admin Panel
 * =============================
 * Full admin control interface with:
 * - Dashboard overview
 * - User management with targeting
 * - Feature flags
 * - App config
 * - Notifications (targeted)
 * - Audit log
 * - System logs
 * - AI Command Bot
 * - Mode toggle (admin ↔ user)
 */
import { useState, useMemo, useCallback } from "react";
import { trpc } from "@/lib/trpc";
import { useAuth } from "@/_core/hooks/useAuth";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogFooter,
} from "@/components/ui/dialog";
import { Textarea } from "@/components/ui/textarea";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { toast } from "sonner";
import {
  Shield, Users, Flag, Settings, Bell, FileText,
  Activity, Bot, ChevronRight, Search, Ban, UserCheck,
  UserX, Crown, Eye, EyeOff, Send, Trash2, RefreshCw,
  AlertTriangle, CheckCircle, Info, XCircle, Megaphone,
  Wrench, BarChart3, Clock, ArrowUpDown, ChevronDown,
} from "lucide-react";

// ─── Types ────────────────────────────────────────
type TargetScope = "individual" | "group" | "all";
type AdminTab = "dashboard" | "users" | "features" | "config" | "notifications" | "audit" | "logs" | "ai";

// ─── Target Selector Component ────────────────────
function TargetSelector({
  scope,
  setScope,
  selectedUserIds,
  setSelectedUserIds,
}: {
  scope: TargetScope;
  setScope: (s: TargetScope) => void;
  selectedUserIds: number[];
  setSelectedUserIds: (ids: number[]) => void;
}) {
  const [searchTerm, setSearchTerm] = useState("");
  const userList = trpc.admin.listUsers.useQuery({ page: 1, limit: 50, search: searchTerm || undefined });

  return (
    <div className="space-y-3 p-3 rounded-lg bg-gray-50 border border-gray-200">
      <Label className="text-sm font-semibold text-gray-700">Target Scope</Label>
      <div className="flex gap-2">
        {(["individual", "group", "all"] as const).map(s => (
          <Button
            key={s}
            variant={scope === s ? "default" : "outline"}
            size="sm"
            onClick={() => { setScope(s); if (s === "all") setSelectedUserIds([]); }}
            className={scope === s ? "bg-blue-600 text-white" : ""}
          >
            {s === "individual" ? "Individual" : s === "group" ? "Group" : "All Users"}
          </Button>
        ))}
      </div>

      {scope !== "all" && (
        <div className="space-y-2">
          <div className="relative">
            <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-gray-400" />
            <Input
              placeholder="Search users..."
              value={searchTerm}
              onChange={e => setSearchTerm(e.target.value)}
              className="pl-9 h-9"
            />
          </div>
          <div className="max-h-40 overflow-y-auto space-y-1">
            {userList.data?.users?.map(u => (
              <label
                key={u.id}
                className={`flex items-center gap-2 p-2 rounded cursor-pointer hover:bg-blue-50 transition-colors ${
                  selectedUserIds.includes(u.id) ? "bg-blue-50 border border-blue-200" : "bg-white border border-gray-100"
                }`}
              >
                <input
                  type={scope === "individual" ? "radio" : "checkbox"}
                  checked={selectedUserIds.includes(u.id)}
                  onChange={() => {
                    if (scope === "individual") {
                      setSelectedUserIds([u.id]);
                    } else {
                      setSelectedUserIds(
                        selectedUserIds.includes(u.id)
                          ? selectedUserIds.filter(id => id !== u.id)
                          : [...selectedUserIds, u.id]
                      );
                    }
                  }}
                  className="accent-blue-600"
                />
                <span className="text-sm font-medium text-gray-800">{u.name || "Unknown"}</span>
                <span className="text-xs text-gray-400 ml-auto">{u.email || u.openId?.slice(0, 8)}</span>
                <Badge variant="outline" className="text-xs">{u.role}</Badge>
              </label>
            ))}
          </div>
          {selectedUserIds.length > 0 && (
            <p className="text-xs text-blue-600 font-medium">{selectedUserIds.length} user(s) selected</p>
          )}
        </div>
      )}
    </div>
  );
}

// ─── Dashboard Tab ────────────────────────────────
function DashboardTab() {
  const dashboard = trpc.admin.dashboard.useQuery(undefined, { refetchInterval: 30000 });
  const d = dashboard.data;

  const stats = [
    { label: "Total Users", value: d?.totalUsers ?? 0, icon: Users, color: "text-blue-600", bg: "bg-blue-50" },
    { label: "Active (7d)", value: d?.activeUsers ?? 0, icon: Activity, color: "text-green-600", bg: "bg-green-50" },
    { label: "Total Trips", value: d?.totalTrips ?? 0, icon: BarChart3, color: "text-purple-600", bg: "bg-purple-50" },
    { label: "Active Trips", value: d?.activeTrips ?? 0, icon: RefreshCw, color: "text-orange-600", bg: "bg-orange-50" },
    { label: "Devices", value: d?.totalDevices ?? 0, icon: Settings, color: "text-cyan-600", bg: "bg-cyan-50" },
    { label: "Alerts", value: d?.totalAlerts ?? 0, icon: AlertTriangle, color: "text-red-600", bg: "bg-red-50" },
  ];

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
        {stats.map(s => (
          <Card key={s.label} className="border border-gray-200 shadow-sm hover:shadow-md transition-shadow">
            <CardContent className="p-4">
              <div className="flex items-center gap-3">
                <div className={`p-2.5 rounded-xl ${s.bg}`}>
                  <s.icon className={`h-5 w-5 ${s.color}`} />
                </div>
                <div>
                  <p className="text-2xl font-bold text-gray-900">{s.value.toLocaleString()}</p>
                  <p className="text-xs text-gray-500">{s.label}</p>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      <Card className="border border-gray-200">
        <CardHeader className="pb-3">
          <CardTitle className="text-sm font-semibold text-gray-700 flex items-center gap-2">
            <Clock className="h-4 w-4" /> Recent Admin Actions
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-2 max-h-64 overflow-y-auto">
            {d?.recentActions?.slice(0, 10).map((a: any) => (
              <div key={a.id} className="flex items-center gap-3 p-2 rounded-lg bg-gray-50 text-sm">
                <Badge variant="outline" className="text-xs shrink-0">{a.actionType}</Badge>
                <span className="text-gray-600 truncate">{a.description}</span>
                <span className="text-xs text-gray-400 ml-auto shrink-0">
                  {new Date(a.createdAt).toLocaleTimeString()}
                </span>
              </div>
            ))}
            {(!d?.recentActions || d.recentActions.length === 0) && (
              <p className="text-sm text-gray-400 text-center py-4">No recent actions</p>
            )}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

// ─── Users Tab ────────────────────────────────────
function UsersTab() {
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState("");
  const [roleFilter, setRoleFilter] = useState<string>("all");
  const [blockScope, setBlockScope] = useState<TargetScope>("individual");
  const [blockUserIds, setBlockUserIds] = useState<number[]>([]);
  const [blockReason, setBlockReason] = useState("");
  const [promoteRole, setPromoteRole] = useState("dispatcher");

  const userList = trpc.admin.listUsers.useQuery({
    page,
    limit: 20,
    search: search || undefined,
    role: roleFilter !== "all" ? roleFilter as any : undefined,
  });

  const blockUser = trpc.admin.blockUser.useMutation({
    onSuccess: (d) => { toast.success(`Blocked ${d.blocked} user(s)`); userList.refetch(); },
  });
  const unblockUser = trpc.admin.unblockUser.useMutation({
    onSuccess: (d) => { toast.success(`Unblocked ${d.unblocked} user(s)`); userList.refetch(); },
  });
  const promoteUser = trpc.admin.promoteUser.useMutation({
    onSuccess: (d) => { toast.success(`Promoted ${d.promoted} user(s) to ${d.role}`); userList.refetch(); },
  });
  const demoteUser = trpc.admin.demoteUser.useMutation({
    onSuccess: (d) => { toast.success(`Demoted ${d.demoted} user(s)`); userList.refetch(); },
  });

  return (
    <div className="space-y-4">
      {/* Search & Filter Bar */}
      <div className="flex gap-3 items-center">
        <div className="relative flex-1">
          <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-gray-400" />
          <Input
            placeholder="Search users by name..."
            value={search}
            onChange={e => { setSearch(e.target.value); setPage(1); }}
            className="pl-9"
          />
        </div>
        <Select value={roleFilter} onValueChange={v => { setRoleFilter(v); setPage(1); }}>
          <SelectTrigger className="w-36">
            <SelectValue placeholder="Role" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All Roles</SelectItem>
            <SelectItem value="admin">Admin</SelectItem>
            <SelectItem value="user">User</SelectItem>
            <SelectItem value="dispatcher">Dispatcher</SelectItem>
            <SelectItem value="driver">Driver</SelectItem>
          </SelectContent>
        </Select>
      </div>

      {/* Action Buttons */}
      <div className="flex gap-2 flex-wrap">
        <Dialog>
          <DialogTrigger asChild>
            <Button variant="outline" size="sm" className="text-red-600 border-red-200 hover:bg-red-50">
              <Ban className="h-3.5 w-3.5 mr-1" /> Block
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Block Users</DialogTitle>
            </DialogHeader>
            <TargetSelector scope={blockScope} setScope={setBlockScope} selectedUserIds={blockUserIds} setSelectedUserIds={setBlockUserIds} />
            <Textarea placeholder="Reason (optional)" value={blockReason} onChange={e => setBlockReason(e.target.value)} />
            <DialogFooter>
              <Button
                variant="destructive"
                onClick={() => blockUser.mutate({ target: { scope: blockScope, userIds: blockUserIds }, reason: blockReason })}
                disabled={blockUser.isPending || (blockScope !== "all" && blockUserIds.length === 0)}
              >
                {blockUser.isPending ? "Blocking..." : "Block"}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>

        <Dialog>
          <DialogTrigger asChild>
            <Button variant="outline" size="sm" className="text-green-600 border-green-200 hover:bg-green-50">
              <UserCheck className="h-3.5 w-3.5 mr-1" /> Unblock
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Unblock Users</DialogTitle>
            </DialogHeader>
            <TargetSelector scope={blockScope} setScope={setBlockScope} selectedUserIds={blockUserIds} setSelectedUserIds={setBlockUserIds} />
            <DialogFooter>
              <Button
                onClick={() => unblockUser.mutate({ target: { scope: blockScope, userIds: blockUserIds } })}
                disabled={unblockUser.isPending || (blockScope !== "all" && blockUserIds.length === 0)}
              >
                {unblockUser.isPending ? "Unblocking..." : "Unblock"}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>

        <Dialog>
          <DialogTrigger asChild>
            <Button variant="outline" size="sm" className="text-purple-600 border-purple-200 hover:bg-purple-50">
              <Crown className="h-3.5 w-3.5 mr-1" /> Promote
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Promote Users</DialogTitle>
            </DialogHeader>
            <TargetSelector scope={blockScope} setScope={setBlockScope} selectedUserIds={blockUserIds} setSelectedUserIds={setBlockUserIds} />
            <Select value={promoteRole} onValueChange={setPromoteRole}>
              <SelectTrigger>
                <SelectValue placeholder="Select role" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="admin">Admin</SelectItem>
                <SelectItem value="dispatcher">Dispatcher</SelectItem>
                <SelectItem value="driver">Driver</SelectItem>
                <SelectItem value="ems">EMS</SelectItem>
                <SelectItem value="sports">Sports</SelectItem>
              </SelectContent>
            </Select>
            <DialogFooter>
              <Button
                onClick={() => promoteUser.mutate({ target: { scope: blockScope, userIds: blockUserIds }, role: promoteRole as any })}
                disabled={promoteUser.isPending || (blockScope !== "all" && blockUserIds.length === 0)}
              >
                {promoteUser.isPending ? "Promoting..." : "Promote"}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>

        <Dialog>
          <DialogTrigger asChild>
            <Button variant="outline" size="sm" className="text-gray-600 border-gray-200 hover:bg-gray-50">
              <UserX className="h-3.5 w-3.5 mr-1" /> Demote
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Demote Users to Regular</DialogTitle>
            </DialogHeader>
            <TargetSelector scope={blockScope} setScope={setBlockScope} selectedUserIds={blockUserIds} setSelectedUserIds={setBlockUserIds} />
            <DialogFooter>
              <Button
                onClick={() => demoteUser.mutate({ target: { scope: blockScope, userIds: blockUserIds } })}
                disabled={demoteUser.isPending || (blockScope !== "all" && blockUserIds.length === 0)}
              >
                {demoteUser.isPending ? "Demoting..." : "Demote to User"}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>

      {/* User Table */}
      <div className="rounded-lg border border-gray-200 overflow-hidden">
        <table className="w-full text-sm">
          <thead className="bg-gray-50">
            <tr>
              <th className="text-left p-3 font-medium text-gray-600">User</th>
              <th className="text-left p-3 font-medium text-gray-600">Role</th>
              <th className="text-left p-3 font-medium text-gray-600">Last Active</th>
              <th className="text-left p-3 font-medium text-gray-600">Status</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-100">
            {userList.data?.users?.map(u => (
              <tr key={u.id} className="hover:bg-gray-50 transition-colors">
                <td className="p-3">
                  <div>
                    <p className="font-medium text-gray-900">{u.name || "Unknown"}</p>
                    <p className="text-xs text-gray-400">{u.email || u.openId?.slice(0, 16)}</p>
                  </div>
                </td>
                <td className="p-3">
                  <Badge
                    variant={u.role === "admin" ? "default" : "outline"}
                    className={u.role === "admin" ? "bg-purple-100 text-purple-700 border-purple-200" : ""}
                  >
                    {u.role}
                  </Badge>
                </td>
                <td className="p-3 text-gray-500 text-xs">
                  {u.lastSignedIn ? new Date(u.lastSignedIn).toLocaleDateString() : "Never"}
                </td>
                <td className="p-3">
                  <div className="w-2 h-2 rounded-full bg-green-400" title="Active" />
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      <div className="flex items-center justify-between">
        <p className="text-xs text-gray-500">
          {userList.data?.total ?? 0} total users
        </p>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" disabled={page <= 1} onClick={() => setPage(p => p - 1)}>
            Previous
          </Button>
          <Button variant="outline" size="sm" onClick={() => setPage(p => p + 1)}>
            Next
          </Button>
        </div>
      </div>
    </div>
  );
}

// ─── Feature Flags Tab ────────────────────────────
function FeaturesTab() {
  const flags = trpc.admin.listFeatureFlags.useQuery();
  const toggleFlag = trpc.admin.toggleFeatureFlag.useMutation({
    onSuccess: () => { toast.success("Feature flag updated"); flags.refetch(); },
  });
  const createFlag = trpc.admin.createFeatureFlag.useMutation({
    onSuccess: () => { toast.success("Feature flag created"); flags.refetch(); },
  });

  const [newKey, setNewKey] = useState("");
  const [newLabel, setNewLabel] = useState("");

  return (
    <div className="space-y-4">
      <div className="flex gap-2">
        <Input placeholder="Key (e.g. dark_mode)" value={newKey} onChange={e => setNewKey(e.target.value)} className="flex-1" />
        <Input placeholder="Label" value={newLabel} onChange={e => setNewLabel(e.target.value)} className="flex-1" />
        <Button
          onClick={() => { createFlag.mutate({ key: newKey, label: newLabel }); setNewKey(""); setNewLabel(""); }}
          disabled={!newKey || !newLabel || createFlag.isPending}
          size="sm"
        >
          Add Flag
        </Button>
      </div>

      <div className="space-y-2">
        {(flags.data ?? []).map((f: any) => (
          <Card key={f.id} className="border border-gray-200">
            <CardContent className="p-4 flex items-center justify-between">
              <div>
                <p className="font-medium text-gray-900">{f.label}</p>
                <p className="text-xs text-gray-400 font-mono">{f.key}</p>
                {f.description && <p className="text-xs text-gray-500 mt-1">{f.description}</p>}
              </div>
              <div className="flex items-center gap-3">
                <Badge variant="outline" className="text-xs">{f.scope}</Badge>
                <Switch
                  checked={f.isEnabled}
                  onCheckedChange={(checked) => toggleFlag.mutate({ key: f.key, isEnabled: checked })}
                />
              </div>
            </CardContent>
          </Card>
        ))}
        {(flags.data ?? []).length === 0 && (
          <p className="text-sm text-gray-400 text-center py-8">No feature flags yet. Add one above.</p>
        )}
      </div>
    </div>
  );
}

// ─── Config Tab ───────────────────────────────────
function ConfigTab() {
  const config = trpc.admin.listConfig.useQuery();
  const updateConfig = trpc.admin.updateConfig.useMutation({
    onSuccess: () => { toast.success("Config updated"); config.refetch(); },
  });
  const setMaintenance = trpc.admin.setMaintenanceMode.useMutation({
    onSuccess: (d) => { toast.success(`Maintenance mode ${d.maintenanceMode ? "enabled" : "disabled"}`); config.refetch(); },
  });

  const [newKey, setNewKey] = useState("");
  const [newValue, setNewValue] = useState("");

  return (
    <div className="space-y-4">
      {/* Maintenance Mode Toggle */}
      <Card className="border-2 border-orange-200 bg-orange-50">
        <CardContent className="p-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Wrench className="h-5 w-5 text-orange-600" />
            <div>
              <p className="font-semibold text-gray-900">Maintenance Mode</p>
              <p className="text-xs text-gray-500">Disable access for all non-admin users</p>
            </div>
          </div>
          <Switch
            onCheckedChange={(checked) => setMaintenance.mutate({ enabled: checked })}
          />
        </CardContent>
      </Card>

      {/* Add Config */}
      <div className="flex gap-2">
        <Input placeholder="Config key" value={newKey} onChange={e => setNewKey(e.target.value)} className="flex-1" />
        <Input placeholder="Value (JSON)" value={newValue} onChange={e => setNewValue(e.target.value)} className="flex-1" />
        <Button
          onClick={() => {
            try {
              const val = JSON.parse(newValue);
              updateConfig.mutate({ key: newKey, value: val });
              setNewKey(""); setNewValue("");
            } catch {
              toast.error("Invalid JSON value");
            }
          }}
          disabled={!newKey || !newValue}
          size="sm"
        >
          Set
        </Button>
      </div>

      {/* Config List */}
      <div className="space-y-2">
        {(config.data ?? []).map((c: any) => (
          <Card key={c.id} className="border border-gray-200">
            <CardContent className="p-4 flex items-center justify-between">
              <div>
                <p className="font-medium text-gray-900 font-mono text-sm">{c.key}</p>
                <p className="text-xs text-gray-500 mt-1">{JSON.stringify(c.value)}</p>
              </div>
              <Badge variant="outline" className="text-xs">{c.category}</Badge>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}

// ─── Notifications Tab ────────────────────────────
function NotificationsTab() {
  const [title, setTitle] = useState("");
  const [message, setMessage] = useState("");
  const [type, setType] = useState<string>("info");
  const [scope, setScope] = useState<TargetScope>("all");
  const [userIds, setUserIds] = useState<number[]>([]);

  const sendNotif = trpc.admin.sendNotification.useMutation({
    onSuccess: () => {
      toast.success("Notification sent!");
      setTitle(""); setMessage("");
    },
  });

  const typeIcons: Record<string, typeof Info> = {
    info: Info,
    warning: AlertTriangle,
    success: CheckCircle,
    error: XCircle,
    announcement: Megaphone,
  };

  return (
    <div className="space-y-4">
      <Card className="border border-gray-200">
        <CardHeader className="pb-3">
          <CardTitle className="text-sm font-semibold text-gray-700">Send Notification</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          <Input placeholder="Title" value={title} onChange={e => setTitle(e.target.value)} />
          <Textarea placeholder="Message body..." value={message} onChange={e => setMessage(e.target.value)} rows={3} />

          <div className="flex gap-2 flex-wrap">
            {["info", "warning", "success", "error", "announcement"].map(t => {
              const Icon = typeIcons[t] ?? Info;
              return (
                <Button
                  key={t}
                  variant={type === t ? "default" : "outline"}
                  size="sm"
                  onClick={() => setType(t)}
                  className={type === t ? "bg-blue-600" : ""}
                >
                  <Icon className="h-3.5 w-3.5 mr-1" />
                  {t}
                </Button>
              );
            })}
          </div>

          <TargetSelector scope={scope} setScope={setScope} selectedUserIds={userIds} setSelectedUserIds={setUserIds} />

          <Button
            className="w-full bg-blue-600 hover:bg-blue-700"
            onClick={() => sendNotif.mutate({
              title, message,
              type: type as any,
              target: { scope, userIds: scope !== "all" ? userIds : undefined },
            })}
            disabled={!title || !message || sendNotif.isPending || (scope !== "all" && userIds.length === 0)}
          >
            <Send className="h-4 w-4 mr-2" />
            {sendNotif.isPending ? "Sending..." : "Send Notification"}
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}

// ─── Audit Log Tab ────────────────────────────────
function AuditTab() {
  const [page, setPage] = useState(1);
  const auditLog = trpc.admin.getAuditLog.useQuery({ page, limit: 30 });

  return (
    <div className="space-y-3">
      <div className="space-y-2 max-h-[500px] overflow-y-auto">
        {auditLog.data?.actions?.map((a: any) => (
          <div key={a.id} className="p-3 rounded-lg bg-gray-50 border border-gray-100 text-sm">
            <div className="flex items-center gap-2 mb-1">
              <Badge variant="outline" className="text-xs">{a.actionType}</Badge>
              <Badge variant="outline" className={`text-xs ${
                a.targetScope === "all" ? "bg-blue-50 text-blue-700" :
                a.targetScope === "group" ? "bg-purple-50 text-purple-700" :
                "bg-gray-50 text-gray-700"
              }`}>
                {a.targetScope}
              </Badge>
              <span className="text-xs text-gray-400 ml-auto">
                {new Date(a.createdAt).toLocaleString()}
              </span>
            </div>
            <p className="text-gray-600">{a.description}</p>
            {a.aiPrompt && (
              <p className="text-xs text-purple-500 mt-1 italic">AI: "{a.aiPrompt}"</p>
            )}
          </div>
        ))}
      </div>
      <div className="flex justify-between items-center">
        <p className="text-xs text-gray-500">{auditLog.data?.total ?? 0} total entries</p>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" disabled={page <= 1} onClick={() => setPage(p => p - 1)}>Prev</Button>
          <Button variant="outline" size="sm" onClick={() => setPage(p => p + 1)}>Next</Button>
        </div>
      </div>
    </div>
  );
}

// ─── System Logs Tab ──────────────────────────────
function LogsTab() {
  const [page, setPage] = useState(1);
  const [level, setLevel] = useState<string>("all");
  const logs = trpc.admin.getSystemLogs.useQuery({
    page,
    limit: 50,
    level: level !== "all" ? level as any : undefined,
  });

  const levelColors: Record<string, string> = {
    debug: "text-gray-400",
    info: "text-blue-600",
    warn: "text-yellow-600",
    error: "text-red-600",
    critical: "text-red-800 font-bold",
  };

  return (
    <div className="space-y-3">
      <div className="flex gap-2">
        {["all", "debug", "info", "warn", "error", "critical"].map(l => (
          <Button
            key={l}
            variant={level === l ? "default" : "outline"}
            size="sm"
            onClick={() => { setLevel(l); setPage(1); }}
            className={level === l ? "bg-blue-600" : ""}
          >
            {l}
          </Button>
        ))}
      </div>

      <div className="font-mono text-xs space-y-1 max-h-[500px] overflow-y-auto bg-gray-900 rounded-lg p-3">
        {logs.data?.logs?.map((l: any) => (
          <div key={l.id} className="flex gap-2">
            <span className="text-gray-500 shrink-0">{new Date(l.createdAt).toLocaleTimeString()}</span>
            <span className={`shrink-0 uppercase w-12 ${levelColors[l.level] ?? "text-gray-400"}`}>{l.level}</span>
            <span className="text-cyan-400 shrink-0">[{l.source}]</span>
            <span className="text-gray-200 break-all">{l.message}</span>
          </div>
        ))}
        {(!logs.data?.logs || logs.data.logs.length === 0) && (
          <p className="text-gray-500 text-center py-4">No logs found</p>
        )}
      </div>

      <div className="flex justify-between items-center">
        <p className="text-xs text-gray-500">{logs.data?.total ?? 0} entries</p>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" disabled={page <= 1} onClick={() => setPage(p => p - 1)}>Prev</Button>
          <Button variant="outline" size="sm" onClick={() => setPage(p => p + 1)}>Next</Button>
        </div>
      </div>
    </div>
  );
}

// ─── AI Command Bot Tab ───────────────────────────
function AICommandTab() {
  const [prompt, setPrompt] = useState("");
  const [history, setHistory] = useState<{ role: "user" | "assistant"; content: string }[]>([]);

  const aiCommand = trpc.admin.aiCommand.useMutation({
    onSuccess: (data) => {
      setHistory(prev => [
        ...prev,
        { role: "assistant", content: data.response },
      ]);
      if (data.executedAction) {
        toast.success("Action Executed: " + JSON.stringify(data.executedAction));
      }
    },
    onError: (err) => {
      toast.error("AI Error: " + err.message);
    },
  });

  const handleSend = useCallback(() => {
    if (!prompt.trim()) return;
    const newHistory = [...history, { role: "user" as const, content: prompt }];
    setHistory(newHistory);
    aiCommand.mutate({ prompt, conversationHistory: history });
    setPrompt("");
  }, [prompt, history, aiCommand]);

  return (
    <div className="flex flex-col h-[500px]">
      {/* Chat History */}
      <div className="flex-1 overflow-y-auto space-y-3 p-3 bg-gray-50 rounded-t-lg border border-gray-200 border-b-0">
        {history.length === 0 && (
          <div className="text-center py-12">
            <Bot className="h-12 w-12 text-blue-300 mx-auto mb-3" />
            <p className="text-gray-500 font-medium">AI Command Bot</p>
            <p className="text-xs text-gray-400 mt-1 max-w-sm mx-auto">
              Tell me what to do. I can block users, send notifications, toggle features, update configs, and more.
            </p>
            <div className="flex flex-wrap gap-2 justify-center mt-4">
              {[
                "Show me all admin users",
                "Block user ID 5",
                "Enable maintenance mode",
                "Send announcement to all users",
              ].map(suggestion => (
                <Button
                  key={suggestion}
                  variant="outline"
                  size="sm"
                  className="text-xs"
                  onClick={() => setPrompt(suggestion)}
                >
                  {suggestion}
                </Button>
              ))}
            </div>
          </div>
        )}

        {history.map((msg, i) => (
          <div key={i} className={`flex ${msg.role === "user" ? "justify-end" : "justify-start"}`}>
            <div className={`max-w-[80%] p-3 rounded-2xl text-sm ${
              msg.role === "user"
                ? "bg-blue-600 text-white rounded-br-md"
                : "bg-white border border-gray-200 text-gray-800 rounded-bl-md shadow-sm"
            }`}>
              <pre className="whitespace-pre-wrap font-sans">{msg.content}</pre>
            </div>
          </div>
        ))}

        {aiCommand.isPending && (
          <div className="flex justify-start">
            <div className="bg-white border border-gray-200 rounded-2xl rounded-bl-md p-3 shadow-sm">
              <div className="flex gap-1">
                <div className="w-2 h-2 rounded-full bg-blue-400 animate-bounce" style={{ animationDelay: "0ms" }} />
                <div className="w-2 h-2 rounded-full bg-blue-400 animate-bounce" style={{ animationDelay: "150ms" }} />
                <div className="w-2 h-2 rounded-full bg-blue-400 animate-bounce" style={{ animationDelay: "300ms" }} />
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Input */}
      <div className="flex gap-2 p-3 bg-white border border-gray-200 rounded-b-lg">
        <Input
          placeholder="Type a command or question..."
          value={prompt}
          onChange={e => setPrompt(e.target.value)}
          onKeyDown={e => { if (e.key === "Enter" && !e.shiftKey) { e.preventDefault(); handleSend(); } }}
          className="flex-1"
        />
        <Button
          onClick={handleSend}
          disabled={!prompt.trim() || aiCommand.isPending}
          className="bg-blue-600 hover:bg-blue-700"
        >
          <Send className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
}

// ─── Main Admin Panel ─────────────────────────────
export default function AdminPanel({ onSwitchToUser }: { onSwitchToUser?: () => void }) {
  const { user } = useAuth();
  const [activeTab, setActiveTab] = useState<AdminTab>("dashboard");

  const tabs: { id: AdminTab; label: string; icon: typeof Shield }[] = [
    { id: "dashboard", label: "Dashboard", icon: BarChart3 },
    { id: "users", label: "Users", icon: Users },
    { id: "features", label: "Features", icon: Flag },
    { id: "config", label: "Config", icon: Settings },
    { id: "notifications", label: "Notify", icon: Bell },
    { id: "audit", label: "Audit", icon: FileText },
    { id: "logs", label: "Logs", icon: Activity },
    { id: "ai", label: "AI Bot", icon: Bot },
  ];

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white border-b border-gray-200 px-6 py-4">
        <div className="flex items-center justify-between max-w-7xl mx-auto">
          <div className="flex items-center gap-3">
            <div className="p-2 rounded-xl bg-gradient-to-br from-blue-500 to-purple-600">
              <Shield className="h-6 w-6 text-white" />
            </div>
            <div>
              <h1 className="text-xl font-bold text-gray-900">Master Admin</h1>
              <p className="text-xs text-gray-500">
                {user?.name ?? "Admin"} — Full System Control
              </p>
            </div>
          </div>
          <div className="flex items-center gap-3">
            <Badge className="bg-purple-100 text-purple-700 border-purple-200">
              <Crown className="h-3 w-3 mr-1" /> Master Admin
            </Badge>
            {onSwitchToUser && (
              <Button
                variant="outline"
                size="sm"
                onClick={onSwitchToUser}
                className="text-blue-600 border-blue-200 hover:bg-blue-50"
              >
                <ArrowUpDown className="h-3.5 w-3.5 mr-1" />
                Switch to User Mode
              </Button>
            )}
          </div>
        </div>
      </div>

      {/* Tab Navigation */}
      <div className="bg-white border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-6">
          <div className="flex gap-1 overflow-x-auto py-2">
            {tabs.map(tab => (
              <Button
                key={tab.id}
                variant="ghost"
                size="sm"
                onClick={() => setActiveTab(tab.id)}
                className={`shrink-0 ${
                  activeTab === tab.id
                    ? "bg-blue-50 text-blue-700 font-semibold"
                    : "text-gray-500 hover:text-gray-700"
                }`}
              >
                <tab.icon className="h-4 w-4 mr-1.5" />
                {tab.label}
              </Button>
            ))}
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="max-w-7xl mx-auto px-6 py-6">
        {activeTab === "dashboard" && <DashboardTab />}
        {activeTab === "users" && <UsersTab />}
        {activeTab === "features" && <FeaturesTab />}
        {activeTab === "config" && <ConfigTab />}
        {activeTab === "notifications" && <NotificationsTab />}
        {activeTab === "audit" && <AuditTab />}
        {activeTab === "logs" && <LogsTab />}
        {activeTab === "ai" && <AICommandTab />}
      </div>
    </div>
  );
}
