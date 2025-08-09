defmodule BudgyWeb.AddressLive.Index do
  use BudgyWeb, :live_view

  alias Budgy.Banking

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        Listing Addresses
        <:actions>
          <.button variant="primary" navigate={~p"/addresses/new"}>
            <.icon name="hero-plus" /> New Address
          </.button>
        </:actions>
      </.header>

      <.table
        id="addresses"
        rows={@streams.addresses}
        row_click={fn {_id, address} -> JS.navigate(~p"/addresses/#{address}") end}
      >
        <:col :let={{_id, address}} label="Street">{address.street}</:col>
        <:col :let={{_id, address}} label="Number">{address.number}</:col>
        <:col :let={{_id, address}} label="Postal code">{address.postal_code}</:col>
        <:col :let={{_id, address}} label="City">{address.city}</:col>
        <:action :let={{_id, address}}>
          <div class="sr-only">
            <.link navigate={~p"/addresses/#{address}"}>Show</.link>
          </div>
          <.link navigate={~p"/addresses/#{address}/edit"}>Edit</.link>
        </:action>
        <:action :let={{id, address}}>
          <.link
            phx-click={JS.push("delete", value: %{id: address.id}) |> hide("##{id}")}
            data-confirm="Are you sure?"
          >
            Delete
          </.link>
        </:action>
      </.table>
    </Layouts.app>
    """
  end

  @impl true
  def mount(_params, _session, socket) do
    {:ok,
     socket
     |> assign(:page_title, "Listing Addresses")
     |> stream(:addresses, Banking.list_addresses())}
  end

  @impl true
  def handle_event("delete", %{"id" => id}, socket) do
    address = Banking.get_address!(id)
    {:ok, _} = Banking.delete_address(address)

    {:noreply, stream_delete(socket, :addresses, address)}
  end
end
