defmodule BudgyWeb.BankLive.Show do
  use BudgyWeb, :live_view

  alias Budgy.Banking

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        Bank {@bank.id}
        <:subtitle>This is a bank record from your database.</:subtitle>
        <:actions>
          <.button navigate={~p"/banks"}>
            <.icon name="hero-arrow-left" />
          </.button>
          <.button variant="primary" navigate={~p"/banks/#{@bank}/edit?return_to=show"}>
            <.icon name="hero-pencil-square" /> Edit bank
          </.button>
        </:actions>
      </.header>

      <.list>
        <:item title="Name">{@bank.name}</:item>
        <:item title="Bic">{@bank.bic}</:item>
        <:item title="Country code">{@bank.country_code}</:item>
      </.list>
    </Layouts.app>
    """
  end

  @impl true
  def mount(%{"id" => id}, _session, socket) do
    {:ok,
     socket
     |> assign(:page_title, "Show Bank")
     |> assign(:bank, Banking.get_bank!(id))}
  end
end
