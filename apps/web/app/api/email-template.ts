export const templateRoutes = {
  list: "/templates",
  create: "/templates",
  byId: (id: string) => `/templates/${id}`,
  update: (id: string) => `/templates/${id}`,
  delete: (id: string) => `/templates/${id}`,
} as const;
