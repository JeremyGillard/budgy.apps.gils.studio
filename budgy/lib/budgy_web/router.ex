defmodule BudgyWeb.Router do
  use BudgyWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {BudgyWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", BudgyWeb do
    pipe_through :browser

    live "/", HomeLive

    live "/addresses", AddressLive.Index, :index
    live "/addresses/new", AddressLive.Form, :new
    live "/addresses/:id", AddressLive.Show, :show
    live "/addresses/:id/edit", AddressLive.Form, :edit

    live "/banks", BankLive.Index, :index
    live "/banks/new", BankLive.Form, :new
    live "/banks/:id", BankLive.Show, :show
    live "/banks/:id/edit", BankLive.Form, :edit

    live "/counterparties", CounterpartyLive.Index, :index
    live "/counterparties/new", CounterpartyLive.Form, :new
    live "/counterparties/:id", CounterpartyLive.Show, :show
    live "/counterparties/:id/edit", CounterpartyLive.Form, :edit
  end

  # Other scopes may use custom stacks.
  # scope "/api", BudgyWeb do
  #   pipe_through :api
  # end

  # Enable LiveDashboard and Swoosh mailbox preview in development
  if Application.compile_env(:budgy, :dev_routes) do
    # If you want to use the LiveDashboard in production, you should put
    # it behind authentication and allow only admins to access it.
    # If your application does not have an admins-only section yet,
    # you can use Plug.BasicAuth to set up some basic authentication
    # as long as you are also using SSL (which you should anyway).
    import Phoenix.LiveDashboard.Router

    scope "/dev" do
      pipe_through :browser

      live_dashboard "/dashboard", metrics: BudgyWeb.Telemetry
      forward "/mailbox", Plug.Swoosh.MailboxPreview
    end
  end
end
