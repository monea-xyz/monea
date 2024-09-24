def run(plan, args):
    plan.print("Stopping specified services")
    services_to_stop = args.get("services", {"to_stop": []}).get("to_stop", [])
    for service_name in services_to_stop:
        plan.stop_service(service_name)
        plan.remove_service(service_name)
