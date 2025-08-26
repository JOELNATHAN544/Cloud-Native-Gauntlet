variable "network_prefix" {
  type        = string
  description = "Network prefix for Vagrant private network (e.g., 192.168.56)"
  default     = "192.168.56"
}

variable "num_workers" {
  type        = number
  description = "Number of worker nodes"
  default     = 1
}


